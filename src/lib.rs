#![allow(non_snake_case)]
use num::traits::{Float, FromPrimitive, ToPrimitive};

pub mod rainflow;

pub fn eq_load<T>(signal: &Vec<T>, m: f64, neq: u64, half: bool) -> Result<f64, rainflow::ExtremaError>
where
    T: Float + FromPrimitive + ToPrimitive + std::fmt::Debug,
{
    let peaktrough = rainflow::get_peaktrough(signal)?;
    let halfcycles = rainflow::get_halfcycles(&peaktrough, half);

    let sum_damage: f64 = halfcycles
        .iter()
        .map(|(_, range)| T::to_f64(range).unwrap().powf(m))
        .sum();
    Ok((sum_damage / (2.0 * neq as f64)).powf(1.0 / m))
}

pub fn eq_load_max_half_cycle_closed(
    signal: &Vec<f64>,
    m: f64,
    neq: u64,
    half: bool,
) -> Result<f64, rainflow::ExtremaError> {
    let peaktrough = rainflow::get_peaktrough_with_rotation(signal)?;
    let halfcycles = rainflow::get_halfcycles(&peaktrough, half);

    let sum_damage: f64 = halfcycles.iter().map(|(_, range)| range.powf(m)).sum();
    Ok((sum_damage / (2.0 * neq as f64)).powf(1.0 / m))
}

mod python_module;
#[cfg(test)]
mod rainflow_tests;
mod tests;
