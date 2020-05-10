//! example_utils
//!
//! provide utility for other modules

use num_complex::Complex;
use num_traits::float::Float;
use num_traits::{Num, NumCast};
use std::f32::consts::PI;

/// cast a numeric value with type T to one with U
pub fn cast_t2u<T, U>(x: T) -> U
where
    T: Num + NumCast,
    U: Num + NumCast,
{
    U::from(x).unwrap()
}

/// return an all-zero vector with the length same as that of the input
pub fn zeros_like<T>(x: &[T]) -> Vec<T> 
where T: Num + NumCast + Copy
{
    vec![cast_t2u::<f32, T>(0.0); x.len()]
}

/// return a scale of the frequency domain
pub fn calc_freq<T>(x: &[T]) -> Vec<T>
where
    T: Float,
{
    assert!(x.len() > 0);
    let df: T = cast_t2u::<f32, T>(1.0) / (x[x.len() - 1] - x[0]);
    (0..x.len()).map(|i| cast_t2u::<usize, T>(i) * df).collect()
}

/// return the power of a complex value
pub fn power<T>(x: &Complex<T>) -> T
where
    T: Num + NumCast + Copy,
{
    x.re * x.re + x.im * x.im
}

/// return a delta value of a vector
pub fn delta<T>(x: &[T]) -> T
where
    T: Num + NumCast + Copy,
{
    match x.len() {
        0 => cast_t2u::<f32, T>(0.0),
        _ => x[1] - x[0],
    }
}

/// return a [`a`, `b`) vector with a step of  `(b-a)/size`.
/// This function is similar to xrange(a, b, size).
pub fn xrange<T>(a: T, b: T, size: usize) -> Vec<T>
where
    T: Num + NumCast + Copy,
{
    let factor: T = (b - a) / cast_t2u::<usize, T>(size);
    (0..size)
        .map(|i| a + cast_t2u::<usize, T>(i) * factor)
        .collect()
}

/// calculate a [`a`, `b`) vector with a step of `step`.
/// This function is similar to numpy.arange(a, b, step).
pub fn arange<T>(a: T, b: T, step: T) -> Vec<T>
where
    T: Num + NumCast + Copy,
{
    let size = cast::usize(cast_t2u::<T, f32>((b - a) / step).floor()).unwrap();
    (0..size)
        .map(|i| a + cast_t2u::<usize, T>(i) * step)
        .collect()
}

/// calculate a [`a`, `b`) vector with a length of `size`.
/// This function is similar to numpy.linspace(a, b, size).
pub fn linspace<T>(a: T, b: T, size: usize) -> Vec<T>
where
    T: Num + NumCast + Copy,
{
    let factor: T = (b - a) / cast_t2u::<usize, T>(size - 1);
    (0..size)
        .map(|i| a + cast_t2u::<usize, T>(i) * factor)
        .collect()
}

/// calculate a sine curve
pub fn sin<T>(t: &[T], f0: T) -> Vec<T>
where
    T: Float,
{
    t.iter()
        .map(|t_| (cast_t2u::<f32, T>(2.0) * cast_t2u::<f32, T>(PI) * f0 * *t_).sin())
        .collect()
}

/// convert a real-value array to a complex32 array
pub fn to_complex<T>(t: &[T]) -> Vec<Complex<T>>
where
    T: Float,
{
    t.iter()
        .map(|t_| Complex::<T> {
            re: *t_,
            im: cast_t2u::<f32, T>(0.0),
        })
        .collect()
}
