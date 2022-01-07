#![allow(non_snake_case)]
use itertools::Itertools;
use num::traits::{Float, FromPrimitive, ToPrimitive};

#[derive(Debug)]
enum Zone {
    Begin,
    Uphill,
    Downhill,
}

pub fn get_peaktrough_thres<T>(signal: &Vec<T>, thres: T) -> Vec<T>
where
    T: Float + FromPrimitive + ToPrimitive + std::fmt::Debug,
{
    let mut peaktrough: Vec<T> = Vec::new();
    let mut zone = Zone::Begin;
    let mut peak = signal[0];
    let mut trough = signal[0];

    for x in signal.iter() {
        match zone {
            Zone::Begin if *x > peak => {
                peak = *x;
                if peak - trough > thres {
                    peaktrough.push(trough);
                    zone = Zone::Uphill;
                }
            }
            Zone::Begin if *x < trough => {
                trough = *x;
                if peak - trough > thres {
                    peaktrough.push(peak);
                    zone = Zone::Downhill;
                }
            }
            Zone::Uphill if *x > peak => peak = *x,
            Zone::Uphill if peak - *x >= thres => {
                peaktrough.push(peak);
                trough = *x;
                zone = Zone::Downhill;
            }
            Zone::Downhill if *x < trough => trough = *x,
            Zone::Downhill if *x - trough >= thres => {
                peaktrough.push(trough);
                peak = *x;
                zone = Zone::Uphill;
            }
            _ => (),
        }
    }
    peaktrough.push(match zone {
        Zone::Uphill => peak,
        Zone::Downhill => trough,
        Zone::Begin => (peak + trough) / T::from(2).unwrap(),
    });
    peaktrough
}

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

pub fn get_halfcycles<T>(peaktrough: &Vec<T>) -> Vec<T>
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
    for (i, s) in S[0..S.len() - 1].iter().enumerate() {
        halfcycles.push((S[i + 1] - *s).abs());
    }

    halfcycles
}

pub fn eq_load<T>(signal: &Vec<T>, m: f64, neq: u64) -> f64
where
    T: Float + FromPrimitive + ToPrimitive + std::fmt::Debug,
{
    let peaktrough = get_peaktrough(&signal);
    let amplitudes = get_halfcycles(&peaktrough);

    let sum_damage: f64 = amplitudes
        .iter()
        .map(|x| T::to_f64(x).unwrap().powf(m))
        .sum();
    let del = (sum_damage / (2.0 * neq as f64)).powf(1.0 / m);
    del
}

mod python_module;
mod tests;
