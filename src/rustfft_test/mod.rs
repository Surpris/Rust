//! rustfft_test
//!
//! provide test functions using rustFFT crate

use cast;
use rulinalg::utils as linalg_utils;
use rustfft::num_complex::Complex32;
use rustfft::num_traits::Zero;
use rustfft::{FFTplanner, FFT};
use std::f32::consts::PI;
use std::sync::Arc;

use super::example_utils;

/// test to use FFTplanner
/// This function follows an example shown in
/// https://docs.rs/rustfft/3.0.1/rustfft/index.html
#[allow(unused_variables)]
pub fn test_fft_vec() {
    let size: usize = 1024;
    let mut input: Vec<Complex32> = vec![Complex32::zero(); size];
    let mut output: Vec<Complex32> = vec![Complex32::zero(); size];

    let mut planner = FFTplanner::<f32>::new(false);
    let fft: Arc<dyn FFT<f32>> = planner.plan_fft(size);
    fft.process(&mut input, &mut output);

    let fft_clone: Arc<dyn FFT<f32>> = Arc::clone(&fft);
}

/// test to apply FFT to a sine curve
pub fn test_fft_sin() {
    // parameters
    let size: usize = 1024;
    let f0: f32 = 60_f32;

    // set arrays
    let t: Vec<f32> = (0..size).map(|i| cast::f32(i) / cast::f32(size)).collect();
    let freq: Vec<f32> = example_utils::calc_freq(&t);
    let mut x: Vec<Complex32> = t
        .clone()
        .iter()
        .map(|t_| Complex32 {
            re: (2.0 * PI * f0 * t_).sin(),
            im: 0.0,
        })
        .collect();

    // apply FFT
    let mut x_f: Vec<Complex32> = vec![Complex32::zero(); size];
    let mut planner = FFTplanner::<f32>::new(false);
    let fft: Arc<dyn FFT<f32>> = planner.plan_fft(size);
    fft.process(&mut x, &mut x_f);

    // extract power spectrum
    let x_f_power: Vec<f32> = x_f
        .iter()
        .map(|x_: &Complex32| example_utils::power(x_))
        .collect();

    // set the target range
    let df = example_utils::delta(&freq);
    let index: Vec<usize> = (0..size)
        .filter(|i| freq[*i] > df && freq[*i] < freq[freq.len() / 2])
        .collect();

    let max_index: usize = linalg_utils::argmax(&x_f_power[index[0]..index[index.len() - 1]]).0;
    let peak_f = freq[index[0]..index[index.len() - 1]][max_index];
    println!("{}, {}", peak_f, f0);
    assert!((peak_f - f0).abs() < df);
}
