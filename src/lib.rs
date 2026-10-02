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

pub fn get_peaktrough_with_rotation<T>(signal: &Vec<T>) -> Vec<T>
where
    T: Float + FromPrimitive + ToPrimitive + std::fmt::Debug,
{
    let max_index = signal
        .iter()
        .enumerate()
        .filter(|(_, &v)| !v.is_nan())
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(i, _)| i)
        .unwrap();

    let mut signal = signal.clone();
    signal.rotate_left(max_index);
    signal.push(signal[0]);

    let peaktrough = get_peaktrough(&signal);

    peaktrough
}

pub fn get_halfcycles<T>(peaktrough: &Vec<T>, half: bool) -> Vec<(T, T)>
where
    T: Float + FromPrimitive + ToPrimitive + std::fmt::Debug,
{
    let mut halfcycles: Vec<(T, T)> = Vec::new();
    // stack-based rewrite of the 4-point rainflow scan: merges only ever touch
    // the top few elements, so removals are O(1) instead of O(n), making this
    // O(n) overall rather than the O(n^2) of repeatedly removing from the middle.
    let mut stack: Vec<T> = Vec::with_capacity(peaktrough.len());

    for &x in peaktrough.iter() {
        stack.push(x);
        while stack.len() >= 4 {
            let n = stack.len();
            if (stack[n - 4] - stack[n - 3]).abs() >= (stack[n - 3] - stack[n - 2]).abs()
                && (stack[n - 2] - stack[n - 1]).abs() >= (stack[n - 3] - stack[n - 2]).abs()
            {
                let mean = (stack[n - 3] + stack[n - 2]) / T::from_f64(2.0).unwrap();
                let range = (stack[n - 3] - stack[n - 2]).abs();
                halfcycles.push((mean, range));
                halfcycles.push((mean, range));
                stack.remove(n - 2);
                stack.remove(n - 3);
            } else {
                break;
            }
        }
    }
    // phase 2
    for w in stack.windows(2) {
        let mean = (w[1] + w[0]) / T::from_f64(2.0).unwrap();
        let r = (w[1] - w[0]).abs();
        halfcycles.push((mean, r));
        if !half {
            halfcycles.push((mean, r));
        }
    }

    halfcycles
}

pub fn eq_load<T>(signal: &Vec<T>, m: f64, neq: u64, half: bool) -> f64
where
    T: Float + FromPrimitive + ToPrimitive + std::fmt::Debug,
{
    let peaktrough = get_peaktrough(&signal);
    let halfcycles = get_halfcycles(&peaktrough, half);

    let sum_damage: f64 = halfcycles
        .iter()
        .map(|(_, range)| T::to_f64(range).unwrap().powf(m))
        .sum();
    let del = (sum_damage / (2.0 * neq as f64)).powf(1.0 / m);
    del
}

pub fn eq_load_max_half_cycle_closed(signal: &Vec<f64>, m: f64, neq: u64, half: bool) -> f64 {
    let peaktrough = get_peaktrough_with_rotation(signal);
    let halfcycles = get_halfcycles(&peaktrough, half);

    let sum_damage: f64 = halfcycles.iter().map(|(_, range)| range.powf(m)).sum();
    let del = (sum_damage / (2.0 * neq as f64)).powf(1.0 / m);
    del
}

mod python_module;
mod tests;
