## fftw 0.6.0
### References
* Docs.rs: https://docs.rs/fftw/0.6.0/fftw/
* GitHub: https://github.com/rust-math/fftw

### functions
* `pub struct Plan<A, B, Plan: PlanSpec>`
* `pub type C2CPlan64 = Plan<c64, c64, Plan64>`
* `pub trait C2CPlan: Sized`
    * `fn aligned(shape: &[usize], sign: Sign, flag: Flag) -> Result<Self>`]
    * `fn c2c(&mut self, in_: &mut [Self::Complex], out: &mut [Self::Complex]) -> Result<()>`
* `pub enum Sign`
* `pub struct Flag: u32`
* `pub struct AlignedVec<T>`
    * `pub fn new(n: usize) -> Self`