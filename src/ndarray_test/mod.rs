//! ndarray_test
//!
//! provide test functions using ndarray crate

use ndarray::{arr1, arr2, Array, Array2, Axis, Slice};

// for Array::zeros
use num_traits::identities as num_id;

// for add, sub, mul ops. of Array
// these ops take `self`.
use std::ops::{Add, Mul, Sub};

/*** http://asukiaaa.blogspot.com/2018/01/rustndarray.html ***/

/// test to call arr1 and arr2
pub fn test_ndarray_call() -> bool {
    let vec1 = arr1(&[1, 2, 3]);
    let mat1 = arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    assert_eq!(vec1.sum(), 6);
    assert_eq!(mat1.shape(), [3, 3]);
    true
}

/// test to call Array
pub fn test_ndarray_array() -> bool {
    let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let arr = Array::from(vec1);
    let mat1 = arr.into_shape((3, 3)).unwrap();
    assert_eq!(mat1, arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]));
    true
}

/// test to use a slice
pub fn test_ndarray_slice() -> bool {
    let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mat1 = Array::from(vec1).into_shape((3, 3)).unwrap();
    assert_eq!(mat1.slice(s![0..2, 1..3]), arr2(&[[2, 3], [5, 6]]));
    true
}

/// test to transpose a matrix
pub fn test_ndarray_transpose() -> bool {
    let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mat1 = Array::from(vec1)
        .into_shape((3, 3))
        .unwrap()
        .reversed_axes();
    assert_eq!(mat1, arr2(&[[1, 4, 7], [2, 5, 8], [3, 6, 9]]));
    true
}

/// test to execute add, sub, dot, mul, etc.
///
/// {add, sub, mul} take `self`, but dot does not take `self.`
pub fn test_ndarray_ops() -> bool {
    let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let vec2 = vec![1, 1, 1, 2, 2, 2, 3, 3, 3];
    let mat1 = Array::from(vec1).into_shape((3, 3)).unwrap();
    let mat2 = Array::from(vec2).into_shape((3, 3)).unwrap();
    // println!("add: {:?}", mat1.add(&mat2));
    println!("add: {:?}", mat1 + &mat2);

    let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mat1 = Array::from(vec1).into_shape((3, 3)).unwrap();
    // println!("sub: {:?}", mat1.sub(&mat2));
    println!("sub: {:?}", mat1 - &mat2);

    let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mat1 = Array::from(vec1).into_shape((3, 3)).unwrap();
    println!("dot: {:?}", mat1.dot(&mat2));

    // let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    // let mat1 = Array::from(vec1).into_shape((3, 3)).unwrap();
    // println!("mul: {:?}", mat1.mul(&mat2));
    println!("mul: {:?}", mat1 * &mat2);
    true
}

/// test to permute a 2D array
///
/// permute is not implemented in ndarray crate in default.
pub fn test_ndarray_permute() -> bool {
    let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let vec2 = vec![2, 3, 1, 5, 6, 4, 8, 9, 7];
    let mat1 = Array::from(vec1).into_shape((3, 3)).unwrap();
    let mat2 = Array::from(vec2).into_shape((3, 3)).unwrap();
    let mat3 = permute_2d(mat1, vec![1, 2, 0], 1);
    assert_eq!(mat3, mat2);
    true
}

/// permute 2D array
fn permute_2d<T>(ary: Array2<T>, order: Vec<usize>, dim: usize) -> Array2<T>
where
    T: Clone + num_id::Zero,
{
    assert!(dim == (0 | 1));
    assert_eq!(
        ary.index_axis(Axis(dim), 0).len(),
        order.len()
    );
    let mut buff: Array2<T> = Array::zeros(ary.raw_dim());
    for ii in 0..order.len() {
        buff.row_mut(ii).assign(&ary.index_axis(Axis(dim), order[ii]));
    }
    match dim {
        0 => buff,
        _ => buff.reversed_axes(),
    }
}

/*** https://qiita.com/osanshouo/items/7342d92a28d2df0b259a ***/

/// test to zeros, ones, from_elem,
pub fn test_ndarray_functions() -> bool {
    true
}
