#![cfg(test)]
use crate::{eq_load, get_halfcycles, get_peaktrough};

static INP: [f64; 20] = [
    40.0, 15.0, 72.0, 22.0, 43.0, 82.0, 75.0, 7.0, 34.0, 49.0, 95.0, 75.0, 85.0, 47.0, 63.0, 31.0,
    90.0, 20.0, 37.0, 39.0,
];

mod tests {
    use super::*;
    #[test]
    fn peaktrough_no_threshold() {
        let ans: [f64; 15] = [
            40.0, 15.0, 72.0, 22.0, 82.0, 7.0, 95.0, 75.0, 85.0, 47.0, 63.0, 31.0, 90.0, 20.0, 39.0,
        ];
        assert!(get_peaktrough(&INP.to_vec())
            .iter()
            .enumerate()
            .all(|(i, &val)| val == ans[i]));
    }

    #[test]
    fn amptest() {
        let ans: [f64; 14] = [
            50.0, 50.0, 10.0, 10.0, 16.0, 16.0, 59.0, 59.0, 25.0, 67.0, 75.0, 88.0, 75.0, 19.0,
        ];
        let peaktroughs = get_peaktrough(&INP.to_vec());
        assert!(get_halfcycles(&peaktroughs, true)
            .iter()
            .enumerate()
            .all(|(i, &val)| val == ans[i]));
    }

    #[test]
    fn eq_load_float() {
        let out = eq_load(&INP.to_vec(), 4.0, 20, true);
        println!("{}", out);
        assert!((out - 46.10943506509813).abs() < 0.0001);
    }
    
    #[test]
    fn eq_load_float_doublecount_residuals() {
        let out = eq_load(&INP.to_vec(), 4.0, 20, false);
        println!("{}", out);
        assert!((out - 53.37846821582299).abs() < 0.0001);
    }
}
