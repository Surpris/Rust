## ndarray 0.13.1
### References
* Docs.rs: https://docs.rs/ndarray/0.13.1/ndarray/
    * For NumPy users, [the instruction provided by the ndarray crate](https://docs.rs/ndarray/0.13.1/ndarray/doc/ndarray_for_numpy_users/index.html) is very useful.
* GitHub: https://github.com/rust-ndarray/ndarray


### generate
* `pub fn arr1<A: Clone>()xs: &[A] -> Array1<A>`
* `pub fn arr2<A: Clone, V: FixedInitializer<Elem = A>>(xs: &[V]) -> Array2<A>`
* `pub type Array<A, D> = ArrayBase<OwnedRepr<A>, D>`
    * `fn from(_: T) -> Self`
    * `pub fn linspace(start: A, end: A, n: usize) -> Self where A: Float`
    * `pub fn ones<Sh>(shape: Sh) -> Self where A: Clone + One, Sh: ShapeBuilder<Dim = D>`
    * `pub fn range(start: A, end: A, step: A) -> Self where A: Float`
    * `pub fn zeros<Sh>(shape: Sh) -> Self where A: Clone + Zero, Sh: ShapeBuilder<Dim = D>`
* `array!`: macro to generate an array

### operate
* `s!`: macro to generate a slice for index
* `pub struct Axis(pub usize)`
* `pub struct Slice`
    * `fn from(_: T) -> Self`
* Array
    * `pub fn index_axis(&self, axis: Axis, index: usize) -> ArrayView<'_, A, D::Smaller> where S: Data, D: RemoveAxis`
    * `pub fn into_shape<E>(self, shape: E) -> Result<ArrayBase<S, E::Dim>, ShapeError> where E: IntoDimension`
    * `pub fn reversed_axes(mut self) -> ArrayBase<S, D>`
    * `pub fn row_mut(&mut self, index: Ix) -> ArrayViewMut1<'_, A> where S: DataMut`
    * `pub fn shape(&self) -> &[usize]`
    * `pub fn slice_axis(&self, axis: Axis, indices: Slice) -> ArrayView<'_, A, D> where S: Data`
    * `pub fn sum(&self) -> A where A: Clone + Add<Output = A> + num_traits::Zero`
* `fn permute_2d<T>(ary: Array2<T>, order: Vec<usize>, dim: usize) -> Array2<T> where T: Clone + num_id::Zero`
    * Self-made function to permute a 2D array

### arithmetic
* `fn add(self, rhs: Rhs) -> Self::Output`
    * `use std::ops::Add;` is required.
* `fn sub(self, rhs: Rhs) -> Self::Output`
    * `use std::ops::Sub;` is required.
* `fn mul(self, rhs: Rhs) -> Self::Output`
    * `use std::ops::Mul;` is required.
* Array
    * `pub fn dot<Rhs>(&self, rhs: &Rhs) -> <Self as Dot<Rhs>>::Output where Self: Dot<Rhs>`