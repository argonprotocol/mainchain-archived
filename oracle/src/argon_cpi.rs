use crate::utils::to_fixed_i128;
use sp_runtime::{traits::One, FixedI128, FixedPointNumber, FixedU128};
use ulx_primitives::ArgonCPI;

pub fn calculate_argon_cpi(us_cpi_ratio: FixedI128, argon_usd_price: FixedU128) -> ArgonCPI {
	let inverted_argon_price =
		to_fixed_i128(argon_usd_price).reciprocal().unwrap() - FixedI128::one();

	us_cpi_ratio + inverted_argon_price
}

#[cfg(test)]
mod test {
	use super::*;
	use sp_runtime::FixedU128;

	#[test]
	fn test_calculate_argon_cpi() {
		let us_cpi_ratio = FixedI128::from_float(0.00);
		let argon_usd_price = FixedU128::from_float(1.00);

		assert_eq!(calculate_argon_cpi(us_cpi_ratio, argon_usd_price), ArgonCPI::from_float(0.0));
	}

	#[test]
	fn rising_argon_price_should_decrease_cpi() {
		let us_cpi_ratio = FixedI128::from_float(0.00);
		let argon_usd_price = FixedU128::from_float(2.00);

		assert_eq!(calculate_argon_cpi(us_cpi_ratio, argon_usd_price), ArgonCPI::from_float(-0.5));
	}

	#[test]
	fn falling_argon_price_should_increase_cpi() {
		let us_cpi_ratio = FixedI128::from_float(0.00);
		let argon_usd_price = FixedU128::from_float(0.75);

		assert_eq!(
			calculate_argon_cpi(us_cpi_ratio, argon_usd_price),
			ArgonCPI::from_rational(1, 3) // 1/0.75 = 1.3333
		);
	}

	#[test]
	fn inflation_in_usd_decreases_cpi() {
		let us_cpi_ratio = FixedI128::from_float(0.01);
		let argon_usd_price = FixedU128::from_float(1.00);

		assert_eq!(calculate_argon_cpi(us_cpi_ratio, argon_usd_price), ArgonCPI::from_float(0.01));
	}

	#[test]
	fn equilibrium_should_have_0_cpi() {
		let us_cpi_ratio = FixedI128::from_float(0.1);
		let argon_usd_price = FixedU128::from_float(1.11);

		assert_eq!(
			calculate_argon_cpi(us_cpi_ratio, argon_usd_price).round(),
			ArgonCPI::from_float(0.0)
		);
	}
}
