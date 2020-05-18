#![feature(test)]

extern crate rust_examples;
extern crate test;
use rust_examples::fftw_test;
use rust_examples::rustfft_test;

static SIZE: usize = 1024 * 1024;
static NUM_TRIAL: usize = 10;
static NUM_THREADS: usize = NUM_TRIAL;

/// benchmark of rustfft in a manner of non-concurrency
/// One result with (SIZE, NUM_TRIAL) = (2**20, 10) is
/// 406,128,940 ns/iter (+/- 22,286,131)
// #[bench]
fn bench_fft_non_concurrency(b: &mut test::Bencher) {
    b.iter(|| {
        rustfft_test::test_fft_non_concurrency(SIZE, NUM_TRIAL);
    });
}

/// benchmark of rustfft in a manner of concurrency
/// One result with (SIZE, NUM_THREADS) = (2**20, 10) is
/// 399,389,820 ns/iter (+/- 10,070,062)
// #[bench]
fn bench_fft_concurrency(b: &mut test::Bencher) {
    b.iter(|| {
        rustfft_test::test_fft_concurrency(SIZE, NUM_THREADS);
    });
}

/// benchmark of rustfft in a manner of concurrency using a thread pool
/// One result with (SIZE, NUM_THREADS) = (2**20, 10) is
/// 446,305,330 ns/iter (+/- 40,416,183)
// #[bench]
fn bench_fft_threadpool(b: &mut test::Bencher) {
    b.iter(|| {
        rustfft_test::test_fft_threadpool(SIZE, NUM_THREADS);
    });
}

/// benchmark of fftw in a manner of non-concurrency
/// One result with (SIZE, NUM_THREADS) = (2**20, 10) is
/// 256,991,460 ns/iter (+/- 35,988,994)
#[bench]
fn bench_fftw_non_concurrency(b: &mut test::Bencher) {
    b.iter(|| {
        fftw_test::test_fft_non_concurrency(SIZE, NUM_THREADS);
    });
}
