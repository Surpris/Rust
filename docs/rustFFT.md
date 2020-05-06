## rustFFT 3.0.1
### References
* Docs.rs: https://docs.rs/rustfft/3.0.1/rustfft/index.html
* GitHub: https://github.com/awelkie/RustFFT

### functions
* `pub struct FFTplanner<T>`
    * `pub fn new(inverse: bool) -> Self`
    * `pub fn plan_fft(&mut self, len: usize) -> Arc<dyn FFT<T>>`
* `Arc<dyn FFT<T>>`
    * `fn process(&self, input: &mut [Complex<T>], output: &mut [Complex<T>])`
        * **This function does not do normalization.**