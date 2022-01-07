use num::traits::{Float, FromPrimitive, ToPrimitive};


pub fn abs_difference<T: std::ops::Sub<Output = T> + std::cmp::PartialOrd>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}

#[derive(Debug)]
enum Zone {
    Begin,
    Uphill,
    Downhill,
}

pub fn get_peaktrough<T>(signal: &Vec<T>, thres: T) -> Vec<T>
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

pub fn get_amplitudes<T>(peaktrough: &Vec<T>) -> Vec<T>
where
    T: Float + FromPrimitive + ToPrimitive + std::fmt::Debug,
{
    let mut amplitudes: Vec<T> = Vec::new();
    let mut S = peaktrough.clone();
    let mut i: usize = 3;

    // phase 1
    while i < S.len() {
        if (S[i - 2] > S[i - 3] && S[i - 1] >= S[i - 3] && S[i] >= S[i - 2])
            || (S[i - 2] < S[i - 3] && S[i - 1] <= S[i - 3] && S[i] <= S[i - 2])
        {
            amplitudes.push(abs_difference(S[i - 2], S[i - 1]));
            amplitudes.push(abs_difference(S[i - 2], S[i - 1]));
            S.remove(i - 1);
            S.remove(i - 2);
        } else {
            i += 1;
        }
    }
    // phase 2
    for (i, s) in S[0..S.len() - 1].iter().enumerate() {
        amplitudes.push(abs_difference(S[i + 1], *s));
    }

    amplitudes
}

pub fn eq_load<T>(signal: &Vec<T>, m: f64, neq: u64) -> f64
where
    T: Float + FromPrimitive + ToPrimitive + std::fmt::Debug,
{
    let peaktrough = get_peaktrough(&signal, T::from(0).unwrap());
    let amplitudes = get_amplitudes(&peaktrough);

    let sum_damage: f64 = amplitudes
        .iter()
        .map(|x| T::to_f64(x).unwrap().powf(m))
        .sum();
    let del = (sum_damage / (2 * neq as f64)).powf(1.0 / m);
    del
}


mod python_module;
mod tests;
