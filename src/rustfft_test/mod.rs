//! rustfft_test
//!
//! provide test functions using rustFFT crate

use rustfft::num_complex::Complex32;
use rustfft::num_traits::Zero;
use rustfft::{FFTplanner, FFT};
use std::sync::Arc;

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
