use anyhow::Result;
use sp_runtime::{traits::One, FixedI128, FixedU128};

#[allow(dead_code)]
pub struct ArgonPriceLookup {
	pub interval: u64,
	pub use_simulated_schedule: bool,
	pub last_price: FixedU128,
	pub last_price_timestamp: u64,
}

impl ArgonPriceLookup {
	pub fn new(use_simulated_schedule: bool, interval: u64) -> Self {
		Self {
			use_simulated_schedule,
			interval,
			last_price: FixedU128::from_u32(1),
			last_price_timestamp: 0,
		}
	}

	/// Calculates the expected cost of an Argon in USD based on the starting and current U.S. CPI.
	pub fn get_target_price(&self, us_cpi_ratio: FixedI128) -> FixedU128 {
		let cpi_as_u128 = FixedI128::one() + us_cpi_ratio;
		FixedU128::from_inner(cpi_as_u128.into_inner() as u128)
	}

	pub async fn get_argon_price(
		&mut self,
		us_cpi_ratio: FixedI128,
		timestamp_millis: u64,
	) -> Result<FixedU128> {
		let price = self.get_latest_price(us_cpi_ratio, timestamp_millis).await?;
		if self.last_price < FixedU128::from_u32(1) {
			self.last_price = FixedU128::from_u32(1);
		}

		self.last_price = price;
		self.last_price_timestamp = timestamp_millis;
		Ok(price)
	}

	#[allow(unused_variables)]
	pub async fn get_latest_price(
		&self,
		us_cpi_ratio: FixedI128,
		timestamp_millis: u64,
	) -> Result<FixedU128> {
		let target_price = self.get_target_price(us_cpi_ratio);
		if self.use_simulated_schedule {
			#[cfg(feature = "fast-runtime")]
			{
				return Ok(self.simulate_price_change(target_price, timestamp_millis))
			}
		}

		// Eventually, we'll want to hit asset hub and moonbeam directly for pricing. Maybe
		// ethereum too if it ends up on there.
		Ok(target_price)
	}
}

#[cfg(feature = "fast-runtime")]
mod dev {
	use chrono::{TimeZone, Timelike};
	use rand::Rng;
	use sp_runtime::{FixedU128, Saturating};

	use crate::argon_price::ArgonPriceLookup;

	impl ArgonPriceLookup {
		pub(crate) fn simulate_price_change(
			&self,
			target_price: FixedU128,
			timestamp_millis: u64,
		) -> FixedU128 {
			let ticks = if self.last_price_timestamp == 0 {
				1
			} else {
				(timestamp_millis - self.last_price_timestamp) / self.interval
			}
			.min(10);
			let mut last_price = self.last_price;
			let tz_offset = chrono::FixedOffset::west_opt(5 * 3600).unwrap();

			let est = tz_offset.timestamp_millis_opt(self.last_price_timestamp as i64).unwrap();
			let one_milligon = FixedU128::from_rational(1, 1000);
			let one_centagon = FixedU128::from_rational(1, 100);

			for _ in 0..ticks {
				match est.hour() {
					0..=6 => {
						// Hold at 1 cent for 15 minutes
						last_price = one_centagon
					},
					7..=8 => {
						// Increase to 1.99 on the minute
						if est.second() < 5 || est.second() > 55 {
							if last_price < FixedU128::from_rational(199, 100) {
								last_price = last_price.saturating_add(one_centagon);
							}
						}
					},
					9..=10 => {
						// Drop back to target
						if last_price > target_price {
							last_price = last_price.saturating_sub(one_centagon);
						}
					},
					11..=13 => {
						match est.minute() {
							// Fluctuate 5 cents up per hour and hold for 15 minutes
							15..=20 => {
								last_price =
									last_price.saturating_add(FixedU128::from_rational(5, 100));
							},
							35..=40 => {
								last_price =
									last_price.saturating_sub(FixedU128::from_rational(5, 100));
							},
							0 | 59 => last_price = target_price,
							_ => {},
						}
					},
					14..=15 => {
						// Randomize price swing for one hour
						let mut rng = rand::thread_rng();
						let direction = rng.gen_range(-1..=1);
						match direction {
							-1 => {
								last_price =
									last_price.saturating_sub(FixedU128::from_rational(5, 100));
							},
							1 => {
								last_price =
									last_price.saturating_add(FixedU128::from_rational(5, 100));
							},
							_ => {},
						}
					},
					16..=18 => {
						// increase 1 milligon per tick
						last_price = last_price.saturating_sub(one_milligon);
					},
					19.. => {
						// Drop 1 cent per tick to 1 cent
						if last_price > one_centagon {
							last_price = last_price.saturating_sub(one_centagon);
						}
					},
				}
			}
			last_price.clamp(FixedU128::from_rational(1, 1000), FixedU128::from_u32(2))
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_get_target_price() {
		let argon_price_lookup = ArgonPriceLookup::new(false, 0);
		let us_cpi_ratio = FixedI128::from_float(0.00);
		assert_eq!(argon_price_lookup.get_target_price(us_cpi_ratio), FixedU128::from_u32(1));
	}

	#[test]
	fn test_get_target_price_with_cpi() {
		let argon_price_lookup = ArgonPriceLookup::new(false, 0);
		let us_cpi_ratio = FixedI128::from_float(0.1);
		assert_eq!(argon_price_lookup.get_target_price(us_cpi_ratio).to_float(), 1.1);
	}

	#[test]
	#[cfg(feature = "fast-runtime")]
	fn can_use_simulated_schedule() {
		use std::time::SystemTime;
		let mut argon_price_lookup = ArgonPriceLookup::new(true, 100);

		argon_price_lookup.last_price = FixedU128::from_float(1.01);
		argon_price_lookup.last_price_timestamp =
			SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64;
		let ts = argon_price_lookup.last_price_timestamp + 1000;
		let price = argon_price_lookup.simulate_price_change(FixedU128::from_u32(1), ts);
		assert_ne!(price, FixedU128::from_u32(1));
	}
}
