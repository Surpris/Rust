//! cpython_test
//!
//! provide test functions using cpython crate

use cpython::{GILGuard, PyDict, PyObject, PyResult, Python};

pub fn hello_world() {
    let gil: GILGuard = Python::acquire_gil();
    hello(gil.python()).unwrap();
}

fn hello(py: Python) -> PyResult<()> {
    let sys = py.import("sys")?;
    let version: String = sys.get(py, "version")?.extract(py)?;

    let locals = PyDict::new(py);
    locals.set_item(py, "os", py.import("os")?)?;
    let user: String = py
        .eval(
            "os.getenv('USER') or os.getenv('USERNAME')",
            None,
            Some(&locals),
        )?
        .extract(py)?;

    println!("Hello {}, I'm Python {}", user, version);
    Ok(())
}

pub fn hello_numpy() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    numpy_array(py).unwrap();
}

fn numpy_array(py: Python) -> PyResult<()> {
    let locals = PyDict::new(py);
    locals.set_item(py, "numpy", py.import("numpy")?)?;
    let _ary: PyObject = py.eval("numpy.array([1,2,3])", None, Some(&locals))?;

    Ok(())
}
