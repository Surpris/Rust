# Rust examples
Repository for studying Rust and developing projects in Rust

# crates to study
* [ndarray](#ndarray)

## ndarray 0.13.1
For NumPy users, [the instruction provided by the ndarray crate](https://docs.rs/ndarray/0.13.1/ndarray/doc/ndarray_for_numpy_users/index.html) is very useful.

### generate
* `pub fn arr1<A: Clone>()xs: &[A] -> Array1<A>`
    * `sum(&self) -> A where A: Clone + Add<Output = A> + num_traits::Zero`
    * `into_shape<E>(self, shape: E) -> Result<ArrayBase<S, E::Dim>, ShapeError> where E: IntoDimension`
* `pub fn arr2<A: Clone, V: FixedInitializer<Elem = A>>(xs: &[V]) -> Array2<A>`
    * `shape(&self) -> &[usize]`
    * `reversed_axes(mut self) -> ArrayBase<S, D>`
* `pub type Array<A, D> = ArrayBase<OwnedRepr<A>, D>`
    * `from(_: T) -> Self`

### operate
* `s!`: slice macro
* `pub struct Axis(pub usize)`
* `pub struct Slice`
    * `fn from(_: T) -> Self`
* Array
    * `pub fn slice_axis(&self, axis: Axis, indices: Slice) -> ArrayView<'_, A, D> where S: Data`

### arithmetic
* `fn add(self, rhs: Rhs) -> Self::Output`
    * `use std::ops::Add;` is required.
* `fn sub(self, rhs: Rhs) -> Self::Output`
    * `use std::ops::Sub;` is required.
* `fn mul(self, rhs: Rhs) -> Self::Output`
    * `use std::ops::Mul;` is required.
* Array
    * `pub fn dot<Rhs>(&self, rhs: &Rhs) -> <Self as Dot<Rhs>>::Output where Self: Dot<Rhs>`