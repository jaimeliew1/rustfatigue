use num::traits::{Float, FromPrimitive, ToPrimitive};
use std::fmt::Debug;

/// Reasons a peak/trough sequence could not be extracted from a signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtremaError {
    EmptySignal,
    AllNaN,
    NonFiniteInClosingSegment,
}

impl std::fmt::Display for ExtremaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ExtremaError::EmptySignal => "signal contains no samples",
            ExtremaError::AllNaN => "signal contains only NaN values",
            ExtremaError::NonFiniteInClosingSegment => {
                "signal contains NaN or infinite values in the segment used to close the largest cycle"
            }
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for ExtremaError {}

fn sign_of<T: Float>(diff: T) -> i32 {
    if diff < T::zero() {
        -1
    } else if diff == T::zero() {
        0
    } else {
        1
    }
}

/// Index of the first (not last) occurrence of the largest finite value, or
/// `None` if every sample is NaN.
fn find_max_index<T: Float>(samples: &[T]) -> Option<usize> {
    let mut best: Option<(usize, T)> = None;

    for (i, &v) in samples.iter().enumerate() {
        if v.is_nan() {
            continue;
        }

        match best {
            Some((_, max)) if v <= max => {}
            _ => best = Some((i, v)),
        }
    }

    best.map(|(i, _)| i)
}

/// Extracts the local peak/trough sequence from a signal, skipping NaN values
/// and collapsing consecutive equal samples.
///
/// When `close_largest_cycle` is true, the scan starts at the largest finite
/// value and wraps around (forward to the end, then from the start back to
/// that value) so the largest half-cycle is closed last. In that wraparound
/// segment, any NaN or infinite sample is reported as an error instead of
/// being silently dropped.
pub fn find_local_extrema<T>(
    samples: &[T],
    close_largest_cycle: bool,
) -> Result<Vec<T>, ExtremaError>
where
    T: Float + FromPrimitive + ToPrimitive + Debug,
{
    if samples.is_empty() {
        return Err(ExtremaError::EmptySignal);
    }

    let mut max_index = if close_largest_cycle {
        match find_max_index(samples) {
            Some(i) => i,
            None => return Err(ExtremaError::AllNaN),
        }
    } else {
        0
    };

    while samples[max_index].is_nan() {
        max_index += 1;
        if max_index == samples.len() {
            return Err(ExtremaError::AllNaN);
        }
    }

    let mut last = samples[max_index];
    let mut last_diff_sign = 0i32;
    let mut extrema: Vec<T> = Vec::with_capacity(samples.len() / 2);

    for &sample in &samples[max_index + 1..] {
        if sample.is_nan() {
            continue;
        }

        let diff_sign = sign_of(sample - last);
        if diff_sign == 0 {
            continue;
        }

        if diff_sign != last_diff_sign {
            extrema.push(last);
        }

        last_diff_sign = diff_sign;
        last = sample;
    }

    if close_largest_cycle {
        for &sample in &samples[..=max_index] {
            if sample.is_nan() || sample.is_infinite() {
                return Err(ExtremaError::NonFiniteInClosingSegment);
            }

            let diff_sign = sign_of(sample - last);
            if diff_sign == 0 {
                continue;
            }

            if diff_sign != last_diff_sign {
                extrema.push(last);
            }

            last_diff_sign = diff_sign;
            last = sample;
        }
    }

    extrema.push(last);

    Ok(extrema)
}

/// Finds the peak/trough sequence of a signal, scanning forward from the start.
pub fn get_peaktrough<T>(signal: &[T]) -> Result<Vec<T>, ExtremaError>
where
    T: Float + FromPrimitive + ToPrimitive + Debug,
{
    find_local_extrema(signal, false)
}

/// Finds the peak/trough sequence of a signal, starting at its largest value
/// and wrapping around so the largest half-cycle is closed last.
pub fn get_peaktrough_with_rotation<T>(signal: &[T]) -> Result<Vec<T>, ExtremaError>
where
    T: Float + FromPrimitive + ToPrimitive + Debug,
{
    find_local_extrema(signal, true)
}

/// Four-point rainflow cycle counting.
///
/// Cycles are pushed as `(mean, range)` tuples: a fully-closed cycle is pushed
/// twice, and the final trailing residual (after the scan) is doubled when
/// `half` is false.
pub fn get_halfcycles<T>(peaktrough: &[T], half: bool) -> Vec<(T, T)>
where
    T: Float + FromPrimitive + ToPrimitive + Debug,
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

