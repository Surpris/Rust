//! rustfft_test
//!
//! provide test functions using rustFFT crate

use rulinalg::utils as linalg_utils;
use rustfft::num_complex::Complex32;
use rustfft::num_traits::Zero;
use rustfft::{FFTplanner, FFT};
use std::sync::{Arc, Mutex};
use std::thread;

use super::example_utils as eutils;
use super::thread_test::ThreadPool;

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
pub fn test_fft_sin(size: usize) {
    // parameters
    let f0: f32 = 60_f32;

    // set arrays
    let t: Vec<f32> = eutils::xrange(0., 1., size);
    let freq: Vec<f32> = eutils::calc_freq(&t);
    let mut x: Vec<Complex32> = eutils::to_complex(&eutils::sin(&t, f0));

    // apply FFT
    let mut x_f: Vec<Complex32> = eutils::zeros_like(&x);
    let mut planner = FFTplanner::<f32>::new(false);
    let fft: Arc<dyn FFT<f32>> = planner.plan_fft(size);
    fft.process(&mut x, &mut x_f);

    // extract power spectrum
    let x_f_power: Vec<f32> = x_f.iter().map(|x_: &Complex32| eutils::power(x_)).collect();

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

/// test to apply FFT in concurrency
/// This function follows an example in
/// https://github.com/awelkie/RustFFT/blob/master/examples/concurrency.rs
pub fn test_fft_concurrency(size: usize, num_threads: usize) {
    // parameters
    let f0: f32 = 60_f32;

    // set arrays
    let t: Vec<f32> = eutils::xrange(0., 1., size);
    let freq: Vec<f32> = eutils::calc_freq(&t);
    let x: Vec<Complex32> = eutils::to_complex(&eutils::sin(&t, f0));

    // apply FFT using multi-threading
    let mut planner = FFTplanner::<f32>::new(false);
    let fft: Arc<dyn FFT<f32>> = planner.plan_fft(size);

    let x_arc = Arc::new(Mutex::new(x));
    let x_f_arc = Arc::new(Mutex::new(Vec::<Vec<Complex32>>::new()));
    let handles: Vec<thread::JoinHandle<_>> = (0..num_threads)
        .map(|_| {
            let fft_ = fft.clone();
            let x_ = x_arc.clone();
            let x_f_ = x_f_arc.clone();
            thread::spawn(move || {
                let x = x_.lock().unwrap();
                let mut x_out: Vec<Complex32> = eutils::zeros_like(&x);
                fft_.process(&mut x.clone(), &mut x_out);

                let mut x_f_ = x_f_.lock().unwrap();
                x_f_.push(x_out);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    // post-process
    let df: f32 = eutils::delta(&freq);
    let index: Vec<usize> = (0..size)
        .filter(|i| freq[*i] > df && freq[*i] < freq[freq.len() / 2])
        .collect();

    for x_f_ in x_f_arc.lock().unwrap().iter() {
        let x_f_power: Vec<f32> = x_f_
            .iter()
            .map(|x_: &Complex32| eutils::power(x_))
            .collect();
        let max_index: usize = linalg_utils::argmax(&x_f_power[index[0]..index[index.len() - 1]]).0;
        let peak_f: f32 = freq.clone()[index[0]..index[index.len() - 1]][max_index];
        println!("{}, {}", peak_f, f0);
        assert!((peak_f - f0).abs() < df);
    }
}

/// test to apply FFT in a manner of non-concurrency
/// This function is to be compared with concurrent one.
pub fn test_fft_non_concurrency(size: usize, num_trial: usize) {
    // parameters
    let f0: f32 = 60_f32;

    // set arrays
    let t: Vec<f32> = eutils::xrange(0., 1., size);
    let freq: Vec<f32> = eutils::calc_freq(&t);
    let x: Vec<Complex32> = eutils::to_complex(&eutils::sin(&t, f0));

    // apply FFT
    let mut planner = FFTplanner::<f32>::new(false);
    let fft: Arc<dyn FFT<f32>> = planner.plan_fft(size);

    let df: f32 = eutils::delta(&freq);
    let index: Vec<usize> = (0..size)
        .filter(|i| freq[*i] > df && freq[*i] < freq[freq.len() / 2])
        .collect();

    for _ii in 0..num_trial {
        let mut x_f: Vec<Complex32> = eutils::zeros_like(&x);
        fft.process(&mut x.clone(), &mut x_f);

        // post-process
        let x_f_power: Vec<f32> = x_f.iter().map(|x_: &Complex32| eutils::power(x_)).collect();

        let max_index: usize = linalg_utils::argmax(&x_f_power[index[0]..index[index.len() - 1]]).0;
        let peak_f: f32 = freq.clone()[index[0]..index[index.len() - 1]][max_index];
        println!("{}, {}", peak_f, f0);
        assert!((peak_f - f0).abs() < df);
    }
}

/// test to apply FFT in concurrency by using ThreadPool
pub fn test_fft_threadpool(size: usize, num_threads: usize) {
    // parameters
    let f0: f32 = 60_f32;

    // set arrays
    let t: Vec<f32> = eutils::xrange(0., 1., size);
    let freq: Vec<f32> = eutils::calc_freq(&t);
    let x: Vec<Complex32> = eutils::to_complex(&eutils::sin(&t, f0));

    // apply FFT using multi-threading
    let mut planner = FFTplanner::<f32>::new(false);
    let fft: Arc<dyn FFT<f32>> = planner.plan_fft(size);

    let x_arc = Arc::new(Mutex::new(x));
    let x_f_arc = Arc::new(Mutex::new(Vec::<Vec<Complex32>>::new()));

    let pool = ThreadPool::new(num_cpus::get());
    for _ in 0..num_threads {
        let fft_ = fft.clone();
        let x_ = x_arc.clone();
        let x_f_ = x_f_arc.clone();
        pool.execute(move || {
            let x = x_.lock().unwrap();
            let mut x_out: Vec<Complex32> = eutils::zeros_like(&x);
            fft_.process(&mut x.clone(), &mut x_out);

            let mut x_f_ = x_f_.lock().unwrap();
            x_f_.push(x_out);
        })
    }

    // post-process
    let df: f32 = eutils::delta(&freq);
    let index: Vec<usize> = (0..size)
        .filter(|i| freq[*i] > df && freq[*i] < freq[freq.len() / 2])
        .collect();

    for x_f_ in x_f_arc.lock().unwrap().iter() {
        let x_f_power: Vec<f32> = x_f_
            .iter()
            .map(|x_: &Complex32| eutils::power(x_))
            .collect();
        let max_index: usize = linalg_utils::argmax(&x_f_power[index[0]..index[index.len() - 1]]).0;
        let peak_f: f32 = freq.clone()[index[0]..index[index.len() - 1]][max_index];
        println!("{}, {}", peak_f, f0);
        assert!((peak_f - f0).abs() < df);
    }
}
