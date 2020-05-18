//! fftw_test
//!
//! provide test functions using fftw crate

use fftw::array::AlignedVec;
use fftw::plan::*;
use fftw::types::*;
use rulinalg::utils as linalg_utils;
use std::f64::consts::PI;

use super::example_utils as eutils;

/// hello fftw
pub fn hello_fftw() {
    let n = 128;
    let mut plan: C2CPlan64 = C2CPlan::aligned(&[n], Sign::Forward, Flag::Measure).unwrap();
    let mut a = AlignedVec::new(n);
    let mut b = AlignedVec::new(n);
    let k0 = 2.0 * PI / n as f64;
    for i in 0..n {
        a[i] = c64::new((k0 * i as f64).cos(), 0.0);
    }
    plan.c2c(&mut a, &mut b).unwrap();
}

/// test to apply FFTW to a sine curve
pub fn test_fft_sin(size: usize) {
    // parameters
    let f0: f32 = 60 as f32;

    // set arrays
    let t: Vec<f32> = eutils::xrange(0., 1., size);
    let freq: Vec<f32> = eutils::calc_freq(&t);
    let mut x: AlignedVec<c32> = AlignedVec::new(size);
    for ii in 0..size {
        x[ii] = c32::new((2.0 * (PI as f32) * f0 * t[ii]).sin(), 0.0);
    }

    // apply FFT
    let mut x_f: AlignedVec<c32> = AlignedVec::new(size);
    let mut planner: C2CPlan32 = C2CPlan::aligned(&[size], Sign::Forward, Flag::Measure).unwrap();
    planner.c2c(&mut x, &mut x_f).unwrap();

    // extract power spectrum
    let x_f_power: Vec<f32> = x_f.iter().map(|x_: &c32| eutils::power(x_)).collect();

    // set the target range
    let df: f32 = eutils::delta(&freq);
    let index: Vec<usize> = (0..size)
        .filter(|i| freq[*i] > df && freq[*i] < freq[freq.len() / 2])
        .collect();

    let max_index: usize = linalg_utils::argmax(&x_f_power[index[0]..index[index.len() - 1]]).0;
    let peak_f: f32 = freq[index[0]..index[index.len() - 1]][max_index];
    println!("{}, {}", peak_f, f0);
    assert!((peak_f - f0).abs() < df);
}

/// test to apply FFTW in a manner of non-concurrency
pub fn test_fft_non_concurrency(size: usize, num_trial: usize) {
    // parameters
    let f0: f32 = 60 as f32;

    // set arrays
    let t: Vec<f32> = eutils::xrange(0., 1., size);
    let freq: Vec<f32> = eutils::calc_freq(&t);
    let mut x: AlignedVec<c32> = AlignedVec::new(size);
    for ii in 0..size {
        x[ii] = c32::new((2.0 * (PI as f32) * f0 * t[ii]).sin(), 0.0);
    }

    // set items for FFT
    let mut planner: C2CPlan32 = C2CPlan::aligned(&[size], Sign::Forward, Flag::Measure).unwrap();

    // set the target range
    let df: f32 = eutils::delta(&freq);
    let index: Vec<usize> = (0..size)
        .filter(|i| freq[*i] > df && freq[*i] < freq[freq.len() / 2])
        .collect();

    for _ii in 0..num_trial {
        let mut x_f: AlignedVec<c32> = AlignedVec::new(size);
        planner.c2c(&mut x.clone(), &mut x_f).unwrap();

        // post-process
        let x_f_power: Vec<f32> = x_f.iter().map(|x_: &c32| eutils::power(x_)).collect();

        let max_index: usize = linalg_utils::argmax(&x_f_power[index[0]..index[index.len() - 1]]).0;
        let peak_f: f32 = freq[index[0]..index[index.len() - 1]][max_index];
        println!("{}, {}", peak_f, f0);
        assert!((peak_f - f0).abs() < df);
    }
}
