//! ndarray
//!
//! provide test functions using ndarray crate

extern crate ndarray;
use ndarray::{arr1, arr2, Array};

pub fn test_ndarray_call() -> bool {
    let vec1 = arr1(&[1, 2, 3]);
    let mat1 = arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    assert_eq!(vec1.sum(), 6);
    assert_eq!(mat1.shape(), [3, 3]);
    true
}

pub fn test_ndarray_Array() -> bool {
    let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let arr = Array::from(vec1);
    let mat1 = arr.into_shape((3, 3)).unwrap();
    assert_eq!(mat1, arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]));
    true
}
