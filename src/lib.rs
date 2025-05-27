#![allow(non_snake_case)]
use itertools::Itertools;
use num::traits::{Float, FromPrimitive, ToPrimitive};

pub fn get_peaktrough<T>(signal: &Vec<T>) -> Vec<T>
where
    T: Float + FromPrimitive + ToPrimitive + std::fmt::Debug,
{
    let mut peaktrough: Vec<T> = Vec::new();
    peaktrough.push(signal[0]);
    for (a, b, c) in signal.iter().tuple_windows() {
        if ((a < b) & (b > c)) | ((a > b) & (b < c)) {
            peaktrough.push(*b);
        }
    }
    peaktrough.push(*signal.last().unwrap());

    peaktrough
}

pub fn get_halfcycles<T>(peaktrough: &Vec<T>, half: bool) -> Vec<T>
where
    T: Float + FromPrimitive + ToPrimitive + std::fmt::Debug,
{
    let mut halfcycles: Vec<T> = Vec::new();
    let mut S = peaktrough.clone();
    let mut i: usize = 3;

    // phase 1
    while i < S.len() {
        if (S[i - 3] - S[i - 2]).abs() >= (S[i - 2] - S[i - 1]).abs()
            && (S[i - 1] - S[i]).abs() >= (S[i - 2] - S[i - 1]).abs()
        {
            halfcycles.push((S[i - 2] - S[i - 1]).abs());
            halfcycles.push((S[i - 2] - S[i - 1]).abs());
            S.remove(i - 1);
            S.remove(i - 2);
        } else {
            i += 1;
        }
    }
    // phase 2
    for w in S.windows(2) {
        let r = (w[1] - w[0]).abs();
        halfcycles.push(r);
        if !half {
            halfcycles.push(r);
        }
    }

    halfcycles
}

pub fn eq_load<T>(signal: &Vec<T>, m: f64, neq: u64, half: bool) -> f64
where
    T: Float + FromPrimitive + ToPrimitive + std::fmt::Debug,
{
    let peaktrough = get_peaktrough(&signal);
    let ranges = get_halfcycles(&peaktrough, half);

    let sum_damage: f64 = ranges
        .iter()
        .map(|x| T::to_f64(x).unwrap().powf(m))
        .sum();
    let del = (sum_damage / (2.0 * neq as f64)).powf(1.0 / m);
    del
}

mod python_module;
mod tests;
