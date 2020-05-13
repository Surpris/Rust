## rust-cpython 0.5.0
### References
* Docs.rs: https://docs.rs/cpython/0.5.0/cpython/
* GitHub: https://github.com/dgrunwald/rust-cpython

### functions
* `pub struct Python<'p>`
    * `pub fn acquire_gil() -> GILGuard`
* `pub struct GILGuard`
    * `pub fn python<'p>(&'p self) -> Python<'p>`
* `pub struct Python<'p>`
    * `pub fn import(self, name: &str) -> PyResult<PyModule>`
    * `pub fn eval(self, code: &str, globals: Option<&PyDict>, locals: Option<&PyDict>) -> PyResult<PyObject>`
* `pub type PyResult<T> = Result<T, PyErr>`
* `pub struct PyDict`
    * `pub fn new(py: Python) -> PyDict`
    * `pub fn set_item<K, V>(&self, py: Python, key: K, value: V) -> PyResult<()> where K: ToPyObject, V: ToPyObject,`