#![cfg(test)]
use crate::rainflow::{find_local_extrema, get_halfcycles, ExtremaError};


static TEST_DATA: [f32; 14] = [
    1.0, 2.0, 2.0, 1.0, 0.0, 1.0, 0.0, -1.0, -2.0, -3.0, 1.0, 0.0, 10.0, -2.0,
];

// get_halfcycles emits cycles in a different order than the reference's stack
// algorithm, so these compare the sorted cycle multiset rather than exact order.
fn sorted(mut cycles: Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    cycles.sort_by(|a, b| a.partial_cmp(b).unwrap());
    cycles
}

#[test]
fn find_local_extrema_half_cycle_open() {
    let expected: [f32; 9] = [1.0, 2.0, 0.0, 1.0, -3.0, 1.0, 0.0, 10.0, -2.0];
    assert_eq!(find_local_extrema(&TEST_DATA, false).unwrap(), expected);
}

#[test]
fn find_local_extrema_max_half_cycle_closed() {
    let expected: [f32; 9] = [10.0, -2.0, 2.0, 0.0, 1.0, -3.0, 1.0, 0.0, 10.0];
    assert_eq!(find_local_extrema(&TEST_DATA, true).unwrap(), expected);
}

#[test]
fn find_local_extrema_breaks_max_ties_on_first_occurrence() {
    // max value (5.0) repeated at indices 1 and 3; the scan must start right after index 1.
    let data: [f32; 7] = [1.0, 5.0, 2.0, 5.0, 3.0, 0.0, 4.0];
    let expected: [f32; 7] = [5.0, 2.0, 5.0, 0.0, 4.0, 1.0, 5.0];
    assert_eq!(find_local_extrema(&data, true).unwrap(), expected);
}

#[test]
fn find_local_extrema_rejects_empty_signal() {
    let data: [f32; 0] = [];
    assert_eq!(find_local_extrema(&data, false), Err(ExtremaError::EmptySignal));
}

#[test]
fn find_local_extrema_rejects_all_nan() {
    let data: [f32; 3] = [f32::NAN, f32::NAN, f32::NAN];
    assert_eq!(find_local_extrema(&data, false), Err(ExtremaError::AllNaN));
}

#[test]
fn find_local_extrema_rejects_non_finite_in_closing_segment() {
    // max value is at index 2; the wraparound segment (indices 0..=2) includes the leading NaN.
    let data: [f32; 4] = [f32::NAN, 1.0, 2.0, 0.5];
    assert_eq!(
        find_local_extrema(&data, true),
        Err(ExtremaError::NonFiniteInClosingSegment)
    );
}

// Fixtures ported from Nordex.Modules.Tests/AlgorithmTests.cs (RainFlowCountNew) to confirm
// get_halfcycles covers the same cases as the reference implementation.

#[test]
fn get_halfcycles_flex() {
    let peaks = find_local_extrema(&TEST_DATA, false).unwrap();
    let expected = sorted(vec![
        (1.5, 1.0),
        (0.5, 1.0),
        (0.5, 1.0),
        (0.5, 1.0),
        (0.5, 1.0),
        (-0.5, 5.0),
        (3.5, 13.0),
        (4.0, 12.0),
    ]);
    assert_eq!(sorted(get_halfcycles(&peaks, true)), expected);
}

#[test]
fn get_halfcycles_constant() {
    let data: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    let peaks = find_local_extrema(&data, false).unwrap();
    assert!(get_halfcycles(&peaks, true).is_empty());
}

#[test]
fn get_halfcycles_with_nan() {
    let data: [f32; 6] = [1.0, 2.0, 1.0, -1.0, f32::NAN, 1.0];
    let peaks = find_local_extrema(&data, false).unwrap();
    let expected = sorted(vec![(1.5, 1.0), (0.5, 3.0), (0.0, 2.0)]);
    assert_eq!(sorted(get_halfcycles(&peaks, true)), expected);
}

#[test]
fn get_halfcycles_with_positive_infinity() {
    let data: [f32; 6] = [1.0, 2.0, 1.0, -1.0, f32::INFINITY, 1.0];
    let peaks = find_local_extrema(&data, false).unwrap();
    let expected = sorted(vec![
        (1.5, 1.0),
        (0.5, 3.0),
        (f32::INFINITY, f32::INFINITY),
        (f32::INFINITY, f32::INFINITY),
    ]);
    assert_eq!(sorted(get_halfcycles(&peaks, true)), expected);
}

#[test]
fn get_halfcycles_with_negative_infinity() {
    let data: [f32; 6] = [1.0, 2.0, 1.0, -1.0, f32::NEG_INFINITY, 1.0];
    let peaks = find_local_extrema(&data, false).unwrap();
    let expected = sorted(vec![
        (1.5, 1.0),
        (f32::NEG_INFINITY, f32::INFINITY),
        (f32::NEG_INFINITY, f32::INFINITY),
    ]);
    assert_eq!(sorted(get_halfcycles(&peaks, true)), expected);
}

#[test]
fn get_halfcycles_ill_formed() {
    // f32::from_bits(1) is the smallest positive subnormal (matches C#'s `float.Epsilon`,
    // not Rust's `f32::EPSILON`, which is the machine epsilon and a much larger value).
    let smallest_subnormal = f32::from_bits(1);

    let data: [f32; 4] = [0.0, 0.0, 0.0, 2.0 * smallest_subnormal];
    let peaks = find_local_extrema(&data, false).unwrap();
    assert_eq!(
        get_halfcycles(&peaks, true),
        vec![(smallest_subnormal, 2.0 * smallest_subnormal)]
    );

    let data2: [f32; 4] = [0.0, 0.0, 0.0, smallest_subnormal];
    let peaks2 = find_local_extrema(&data2, false).unwrap();
    assert_eq!(get_halfcycles(&peaks2, true), vec![(0.0, smallest_subnormal)]);
}

#[test]
fn get_halfcycles_max_half_cycle_closed() {
    let peaks = find_local_extrema(&TEST_DATA, true).unwrap();
    let expected = sorted(vec![
        (0.5, 1.0),
        (0.5, 1.0),
        (0.0, 4.0),
        (0.0, 4.0),
        (0.5, 1.0),
        (0.5, 1.0),
        (3.5, 13.0),
        (3.5, 13.0),
    ]);
    assert_eq!(sorted(get_halfcycles(&peaks, true)), expected);
}

