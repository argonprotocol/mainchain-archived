use sp_runtime::{
	traits::{CheckedDiv, One, Zero},
	FixedI128, FixedPointNumber, FixedU128,
};

use ulx_primitives::ArgonCPI;

pub fn calculate_argon_cpi(target_argon_price: FixedU128, argon_usd_price: FixedU128) -> ArgonCPI {
	let ratio = target_argon_price.checked_div(&argon_usd_price).unwrap_or(FixedU128::zero());
	ArgonCPI::from_inner(ratio.into_inner() as i128) - FixedI128::one()
}

#[cfg(test)]
mod test {
	use sp_runtime::FixedU128;

	use super::*;

	#[test]
	fn price_below_target_means_deflation() {
		let target_price = FixedU128::from_float(1.10);
		let argon_usd_price = FixedU128::from_float(1.00);

		assert!(calculate_argon_cpi(target_price, argon_usd_price).is_positive());
	}

	#[test]
	fn price_above_target_means_inflation() {
		let target_price = FixedU128::from_float(1.10);
		let argon_usd_price = FixedU128::from_float(1.15);

		assert!(calculate_argon_cpi(target_price, argon_usd_price).is_negative());
	}

	#[test]
	fn equilibrium_should_have_0_cpi() {
		let target_price = FixedU128::from_float(1.15);
		let argon_usd_price = FixedU128::from_float(1.15);

		assert_eq!(
			calculate_argon_cpi(target_price, argon_usd_price).round(),
			ArgonCPI::from_float(0.0)
		);
	}
}
