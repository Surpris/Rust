//! fft_utils
//!
//! provide utility for other modules

use num_complex::Complex32;
use rustfft::FFTnum;

/// calculate a scale of the frequency domain
pub fn calc_freq(x: &[f32]) -> Vec<f32> {
    assert!(x.len() > 0);
    let df: f32 = 1.0 / (x[x.len() - 1] - x[0]);
    (0..x.len()).map(|i| cast::f32(i) * df).collect()
}

/// calculate the power of a complex value
pub fn power(x: &Complex32) -> f32 {
    f32::powf(x.re, 2.0) + f32::powf(x.im, 2.0)
}

/// extract a delta value of a vector
pub fn delta<T>(x: &[T]) -> T
where
    T: FFTnum,
{
    assert!(x.len() > 0);
    x[1] - x[0]
}
