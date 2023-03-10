#![allow(unused_assignments)]

use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};
// use crate::cryptography::type_traits::{MyAdd, MyMul};
use crate::tensor_library::errors::MatrixError;
use crate::tensor_library::layout::Layout;
use crate::tensor_library::utils::{calc_concat_shape, calc_strides_from_shape, check_concat_dims};


#[derive(Debug, Clone)]
pub struct Matrix<T> where T: Clone + Default + 'static{
    pub shape: Vec<usize>,
    pub strides: Vec<usize>,
    pub data: Vec<T>,
    pub layout: Layout,
    pub size: usize
}

impl<T> Matrix<T> where T: Clone + Default {
    /// Constructs a new, non-empty Matrix<T> where cells are set to `T::default`.
    /// Use `Matrix::from_iter` if you want to set the matrix from an iterator.
    ///
    /// # Panics
    /// Panics if shape vector is empty
    /// Panics if data size provided through iterator isn't equal to the size defined by the shape
    ///
    /// # Example
    /// ```
    /// use Cryptonic::tensor_library::layout::Layout;
    /// use Cryptonic::tensor_library::matrix::Matrix;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::RowMajor);
    /// ```
    pub fn new(_shape: Vec<usize>, layout: Layout) -> Matrix<T>
        where
            T: Default,
    {
        Matrix::from_iter(_shape, (0..).map(|_| T::default()), layout)
    }

    /// Constructs a new, non-empty Matrix<T> where cells are set from an iterator
    ///
    /// # Panics
    /// Panics if shape vector is empty
    /// Panics if data size provided through iterator isn't equal to the size defined by the shape
    ///
    /// # Example
    /// ```
    /// use Cryptonic::{matrix::Matrix, layout::Layout};
    /// use Cryptonic::tensor_library::layout::Layout;
    /// use Cryptonic::tensor_library::matrix::Matrix;
    /// let mut mat: Matrix<i32> = Matrix::from_iter(vec![3, 4], 0.., Layout::RowMajor);
    /// ```
    ///
    pub fn from_iter(
        _shape: Vec<usize>,
        _data: impl IntoIterator<Item=T>,
        _layout: Layout,
    ) -> Matrix<T> {
        assert!(!_shape.is_empty());
        let _temp_shape = _shape.clone();
        Matrix {
            shape: _shape,
            strides: {
                let mut data_size: usize = 1;
                let mut strides: Vec<usize> = vec![0; _temp_shape.len()];

                if _layout == Layout::RowMajor {
                    for i in (1..(_temp_shape.len() + 1)).rev() {
                        strides[i - 1] = data_size;
                        data_size = strides[i - 1] * _temp_shape[i - 1];
                    }
                }
                // For Column Major Layout
                else {
                    for i in 0.._temp_shape.len() {
                        strides[i] = data_size;
                        data_size = strides[i] * _temp_shape[i];
                    }
                }
                //Return Strides
                strides
            },
            data: {
                let data: Vec<_> = _data
                    .into_iter()
                    .take(_temp_shape.iter().copied().reduce(|a, b| a * b).unwrap())
                    .collect();
                assert_eq!(
                    data.len(),
                    _temp_shape.iter().copied().reduce(|a, b| a * b).unwrap()
                );
                data
            },
            layout: _layout,
            size: _temp_shape.iter().copied().reduce(|a, b| a * b).unwrap()
        }
    }
}
// Implements getters and setters for fields other than the data.
impl<T> Matrix<T>  where T: Clone + Default {
    /// Returns the full size of a matrix
    ///
    /// # Examples
    /// ```
    /// use Cryptonic::tensor_library::layout::Layout;
    /// use Cryptonic::tensor_library::matrix::Matrix;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::RowMajor);
    /// println!("{}", mat.size()); // Prints 12, because 12 = 3*4
    /// ```
    pub fn size(&self) -> usize {
        self.shape.iter().copied().reduce(|a, b| a * b).unwrap()
    }

    /// Returns the shape of the matrix
    ///
    /// # Examples
    /// ```
    /// use Cryptonic::tensor_library::layout::Layout;
    /// use Cryptonic::tensor_library::matrix::Matrix;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::RowMajor);
    /// println!("{:?}", mat.shape()); // Prints [3, 4]
    /// ```
    pub fn shape(&self) -> &Vec<usize> {
        &self.shape
    }

    /// Reshapes the matrix if possible.
    ///
    /// # Examples
    /// ```
    /// use Cryptonic::tensor_library::errors::MatrixError;
    /// use Cryptonic::tensor_library::layout::Layout;
    /// use Cryptonic::tensor_library::matrix::Matrix;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![100], Layout::RowMajor);
    ///
    /// assert_eq!(Err(MatrixError::ReshapeError), mat.reshape(&vec![20, 6]));
    /// let l = mat.reshape(&vec![20, 5]);
    /// assert_eq!(mat.shape(), &vec![20, 5]);
    /// ```
    pub fn reshape(&mut self, new_shape: &Vec<usize>) -> Result<(), MatrixError> {
        let size: usize = new_shape.iter().copied().reduce(|a, b| a * b).unwrap();
        if size == self.size {
            self.shape = new_shape.clone();
            self.strides = calc_strides_from_shape(new_shape, self.layout);
            Ok(())
        } else {
            Err(MatrixError::ReshapeError)
        }
    }

    /// Returns the strides of the matrix
    /// This depends on the layout of the matrix(i.e. whether it's Row Major or Column Major)
    /// # Examples
    /// ```
    /// use Cryptonic::tensor_library::layout::Layout;
    /// use Cryptonic::tensor_library::matrix::Matrix;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::RowMajor);
    /// println!("{:?}", mat.strides()); // Prints [4, 1]
    /// ```
    /// ```
    /// use Cryptonic::tensor_library::layout::Layout;
    /// use Cryptonic::tensor_library::matrix::Matrix;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::ColumnMajor);
    /// println!("{:?}", mat.strides()); // Prints [1, 3]
    /// ```
    pub fn strides(&self) -> &Vec<usize> {
        &self.strides
    }

    /// Directly changes the strides of the matrix. This should be used with extreme caution
    /// mostly with the broadcast() function since it calculates the rest and should be used in
    /// combination with set_shape().
    ///
    /// # Examples
    /// ```
    /// use Cryptonic::tensor_library::layout::Layout;
    /// use Cryptonic::tensor_library::matrix::Matrix;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::RowMajor);
    /// mat.set_strides(&vec![3, 4]);
    /// println!("{:?}", mat.strides()); // Prints [3, 4]
    /// ```
    pub fn set_strides(&mut self, _strides: &[usize]) {
        self.strides = (*_strides.to_owned()).to_owned();
    }

    /// Directly changes the shape of the matrix. This should be used with extreme caution
    /// mostly with the broadcast() function since it calculates the rest and should be used in
    /// combination with set_strides(). Essentially it's the same as reshape() but it doesn't
    /// recalculate or check the strides which is why it's so dangerous.
    /// # Examples
    /// ```
    /// use Cryptonic::tensor_library::layout::Layout;
    /// use Cryptonic::tensor_library::matrix::Matrix;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::RowMajor);
    /// mat.set_shape(&vec![12]);
    /// println!("{:?}", mat.shape()); // Prints [3, 4]
    /// ```
    pub fn set_shape(&mut self, _shape: &[usize]) {
        self.shape = (*_shape.to_owned()).to_owned();
    }
}

// Implements functions that are directly used by other functions for example check_bounds()
// shouldn't be used directly by API users which is why in the future it might go private.
// The same goes for get_physical_id()
impl<T> Matrix<T>  where T: Clone + Default {
    /// This is a utilities function, which is used any time a index is given as an input.
    /// Takes a idx: Vec<usize> and checks that the number of dimensions is the same as the
    /// matrix. Then for each element in idx it's checked that it's smaller than its corresponding
    /// element in &self.shape.
    ///
    /// # Examples
    /// ```
    /// use Cryptonic::tensor_library::layout::Layout;
    /// use Cryptonic::tensor_library::matrix::Matrix;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4],Layout::RowMajor);
    /// println!("{:?}", mat.check_bounds(&vec![3, 4]).err()); // Prints Error because !3<3 && !4<4
    /// println!("{:?}", mat.check_bounds(&vec![2, 3]).unwrap()); // Prints () because 2<3 && 3<4
    /// ```
    pub fn check_bounds(&self, idx: &Vec<usize>) -> Result<bool, MatrixError> {
        if idx.len() != self.shape.len() {
            return Err(MatrixError::DimError);
        }
        match !idx.iter().zip(self.shape.iter()).any(|(x, y)| x >= y) {
            true => Ok(true),
            false => Err(MatrixError::OutOfBounds),
        }
    }

    /// Returns the physical id in the self.data vector
    /// Returns None if check_bounds is false
    ///
    ///
    ///
    /// # Examples
    /// ```
    /// use Cryptonic::tensor_library::layout::Layout;
    /// use Cryptonic::tensor_library::matrix::Matrix;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::RowMajor);
    /// println!("{}", mat.get_physical_idx(&vec![2, 1]).unwrap()); // Prints 9, because 9 = 2*4 + 1*1, since strides = [4, 1]
    /// ```
    pub fn get_physical_idx(&self, idx: &Vec<usize>) -> Result<usize, MatrixError> {
        let mut return_val: usize = 0;
        match self.check_bounds(idx) {
            Ok(_) => {
                for (i, index) in idx.iter().enumerate() {
                    return_val += index * self.strides[i];
                }
                Ok(return_val)
            }
            // This broadcasts the error from self.check_bounds()
            Err(err) => Err(err),
        }
    }
}

// Implements the functions which get and set data.
impl<T> Matrix<T>  where T: Clone + Default {
    /// Returns a non-mutable reference to a specific element in the matrix. On input an idx, we
    /// bound_check it and then return the element at that index.
    ///
    /// # Examples
    /// ```
    /// use Cryptonic::tensor_library::layout::Layout;
    /// use Cryptonic::tensor_library::matrix::Matrix;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::RowMajor);
    /// println!("{}", mat.get(&vec![2, 1]).unwrap()); // prints 0 because i32 default value is 0
    /// println!("{:?}", mat.get(&vec![5, 6]).err()); // prints Error because self.get_physical_idx() fails
    /// ```
    // TODO: Add slicing
    pub fn get(&self, idx: &Vec<usize>) -> Result<&T, MatrixError> {
        match self.get_physical_idx(idx) {
            Ok(physical_idx) => Ok(&self.data[physical_idx]),
            Err(m_err) => Err(m_err),
        }
    }

    /// Same as self.get(), but returns a copy. Since the functionality is the same
    /// I haven't included examples
    ///
    // TODO: Add slicing
    pub fn get_copy(&self, idx: &Vec<usize>) -> Result<T, MatrixError> {
        match self.get_physical_idx(idx) {
            Ok(physical_idx) => Ok(self.data[physical_idx].clone()),
            Err(m_err) => Err(m_err),
        }
    }

    /// Same as self.get(), but returns a mutable reference. Since the functionality is the same
    /// I haven't included examples
    ///
    // TODO: Add slicing
    pub fn get_mut(&mut self, idx: &Vec<usize>) -> Result<&mut T, MatrixError> {
        match self.get_physical_idx(idx) {
            Ok(physical_idx) => Ok(&mut self.data[physical_idx]),
            Err(m_err) => Err(m_err),
        }
    }

    /// Sets a specific value in the matrix to the input. This is where self.get_mut gets used. In
    /// the future we might make get_mut private.
    /// Again the index gets bounds checked. And the value gets put if the bounds check succeeds.
    ///
    /// In the future we'll allow the replacement of entire slices or we'll use a specific concat
    /// function for that case.
    /// # Examples
    /// ```
    /// use Cryptonic::tensor_library::layout::Layout;
    /// use Cryptonic::tensor_library::matrix::Matrix;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::RowMajor);
    /// mat.set(&vec![0,0], 5);
    /// println!("{}", mat.get(&vec![0,0]).unwrap()); // print 5
    /// ```
    // TODO: Add slicing
    pub fn set(&mut self, idx: &Vec<usize>, value: T) -> Result<(), MatrixError> {
        match self.get_mut(idx) {
            Ok(cell) => {
                *cell = value;
                Ok(())
            }
            Err(m_err) => Err(m_err),
        }
    }

    /// Same as self.get(), but returns a copy of an entire row.
    // TODO: Add examples
    // TODO: Add slicing
    pub fn get_copy_row(&self, idx: &mut Vec<usize>) -> Result<Vec<T>, MatrixError> {
        if idx.len() != self.shape().len() - 1 {
            return Err(MatrixError::DimError);
        }
        let mut result = vec![T::default(); *self.shape().last().unwrap()];
        for (i, item) in result.iter_mut().enumerate() {
            idx.push(i);
            match self.get_physical_idx(idx) {
                Ok(physical_idx) => {
                    *item = self.data[physical_idx].clone();
                },
                Err(m_err) => {
                    return Err(m_err);
                },
            }
            idx.pop();
        }
        Ok(result)
    }
}

// Implements the apply and apply_mut methods.
impl<T> Matrix<T>  where T: Clone + Default {
    /// Apply a function to all cells of the matrix.
    /// Cells are provided as immutable references to the function,
    /// if you want to modify the cells, use `apply_mut`.
    ///
    /// # Examples
    /// ```
    /// // Get the sum of all cells
    /// use Cryptonic::tensor_library::layout::Layout;
    /// use Cryptonic::tensor_library::matrix::Matrix;
    /// let mut mat: Matrix<i32> = Matrix::from_iter(vec![3, 6], 1..,Layout::RowMajor);
    /// let mut sum = 0;
    /// mat.apply(|n| sum += *n);
    ///
    /// assert_eq!(sum, 171);
    /// ```
    // TODO: Once slices are added allow apply on specific slices
    pub fn apply<F: FnMut(&T)>(&self, func: F) {
        self.data.iter().for_each(func);
    }

    /// Apply a function to all cells of the matrix.
    /// Cells are provided as mutable references to the function,
    /// and can therefore be modified.
    ///
    /// # Examples
    /// ```
    /// // Modify all cells with a function
    /// use Cryptonic::tensor_library::layout::Layout;
    /// use Cryptonic::tensor_library::matrix::Matrix;
    /// let mut mat: Matrix<i32> = Matrix::from_iter(vec![3, 4], 1..,Layout::RowMajor);
    /// mat.apply_mut(|mut n| *n *= 2);
    ///
    /// assert_eq!(mat.get(&vec![0, 0]).unwrap(), &2);
    /// assert_eq!(mat.get(&vec![0, 1]).unwrap(), &4);
    /// assert_eq!(mat.get(&vec![0, 2]).unwrap(), &6);
    /// ```
    /// TODO: Once slices are added allow apply on specific slices
    pub fn apply_mut<F: FnMut(&mut T)>(&mut self, func: F) {
        self.data.iter_mut().for_each(func);
    }
}

// Implements structure changing methods
impl<T> Matrix<T>  where T: Clone + Default {
    /// Transposes the matrix. Reverses the shape, strides and in turn switches the layout.
    ///
    /// # Examples
    /// ```
    /// // Modify all cells with a function
    /// use Cryptonic::tensor_library::layout::Layout;
    /// use Cryptonic::tensor_library::matrix::Matrix;
    /// let mut mat: Matrix<i32> = Matrix::from_iter(vec![3, 4], 1..,Layout::RowMajor);
    /// mat.transpose();
    ///
    /// println!("{:?}", mat.shape()); // prints [4, 3]
    /// println!("{:?}", mat.strides()); // prints [4, 3]
    /// println!("{:?}", mat.layout); // prints Layout::ColumnMajor
    /// ```
    pub fn transpose(&mut self) {
        self.shape.reverse();
        self.strides.reverse();
        match self.layout {
            Layout::RowMajor => self.layout = Layout::ColumnMajor,
            Layout::ColumnMajor => self.layout = Layout::RowMajor,
        }
    }

    /// Transposes the matrix. Reverses the shape, strides and in turn switches the layout.
    ///
    /// # Examples
    /// ```
    /// // Modify all cells with a function
    /// use Cryptonic::tensor_library::layout::Layout;
    /// use Cryptonic::tensor_library::matrix::Matrix;
    /// let mut mat: Matrix<i32> = Matrix::from_iter(vec![3, 4], 1..,Layout::RowMajor);
    /// mat.flatten();
    ///
    /// println!("{:?}", mat.shape()); // prints [12]
    /// println!("{:?}", mat.strides()); // prints [1]
    /// println!("{:?}", mat.layout); // prints Layout::ColumnMajor
    /// ```
    pub fn flatten(&mut self){
        match self.reshape(&vec![self.size()]) {
            Ok(_) => {},
            Err(err) => panic!("{}", err)
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////


/// Given two matrices the function broadcast either makes their shapes compatible or returns
/// an error stating that the given matrices aren't broadcastable.
///
/// # Examples
/// ```
/// use Cryptonic::tensor_library::layout::Layout;
/// use Cryptonic::tensor_library::matrix::{broadcast, Matrix};
/// match broadcast(&vec![3, 4], Layout::RowMajor, &vec![7, 3, 4], Layout::RowMajor) {
///             Ok((v1, v2, v3)) => {
///                 println!("{:?}", v1); // Should print out [7, 3, 4]
///                 println!("{:?}", v2); // Should print out [0, 4, 1]
///                 println!("{:?}", v3); // Should print out [12, 4, 1]
///             }
///             Err(E) => panic!("{}", E)
///         }
/// ```
///
/// TODO: Add tests
///
/// This was added due to clippy warnings
type BroadcastRetType = Result<(Vec<usize>, Vec<usize>, Vec<usize>), MatrixError>;

pub fn broadcast(
    lhs_shape: &Vec<usize>,
    lhs_layout: Layout,
    rhs_shape: &Vec<usize>,
    rhs_layout: Layout,
) -> BroadcastRetType {
    let lhs_shape = if lhs_shape.len() < rhs_shape.len() {
        let ones = vec![1; rhs_shape.len() - lhs_shape.len()];
        [&ones[..], &lhs_shape[..]].concat()
    } else {
        lhs_shape.clone()
    };

    let rhs_shape = if rhs_shape.len() < lhs_shape.len() {
        let ones = vec![1; lhs_shape.len() - rhs_shape.len()];
        [&ones[..], &rhs_shape[..]].concat()
    } else {
        rhs_shape.clone()
    };

    let mut broadcasted_shape: Vec<usize> = Vec::with_capacity(lhs_shape.len());
    let mut broadcasted_lhs_strides: Vec<usize> = calc_strides_from_shape(&lhs_shape, lhs_layout);
    let mut broadcasted_rhs_strides: Vec<usize> = calc_strides_from_shape(&rhs_shape, rhs_layout);

    for (i, (&lhs, &rhs)) in lhs_shape.iter().zip(rhs_shape.iter()).enumerate() {
        if lhs == rhs {
            broadcasted_shape.push(lhs);
        } else if lhs == 1 {
            broadcasted_shape.push(rhs);
            broadcasted_lhs_strides[i] = 0;
        } else if rhs == 1 {
            broadcasted_shape.push(lhs);
            broadcasted_rhs_strides[i] = 0;
        } else {
            return Err(MatrixError::BroadcastError);
        }
    }

    Ok((
        broadcasted_shape,
        broadcasted_lhs_strides,
        broadcasted_rhs_strides,
    ))
}


/// Given two matrices the function concatenates them if they comply with the check_concat_dims()
/// function or returns an error stating that the given matrices differ by more than just the axis.
/// For up,down,left and right concat change the place of lhs and rhs or first transpose them. It's
/// up to the API user how he feeds the data to us.
///
/// The method takes ownership of rhs and lhs for it's duration and then returns it. In the future
/// we'll most likely add a feature to take them by reference.
/// # Examples
/// ```
/// use Cryptonic::tensor_library::layout::Layout;
/// use Cryptonic::tensor_library::matrix::{concat, Matrix, MatrixIter};
/// let mut rhs: Matrix<i32> = Matrix::from_iter(vec![2, 2], 1..,Layout::RowMajor);
/// let mut lhs: Matrix<i32> = Matrix::from_iter(vec![2, 4], 1..,Layout::RowMajor);
///
/// let mut x = concat(rhs, lhs, 1);
/// match x {
///     Ok((val, _rhs, _lhs)) => {
///         let mut matrix_iter = MatrixIter {
///             mat: &val,
///             index: vec![0; val.shape().len()],
///             current_el: None,
///             empty: false,
///         };
///         for (item, idx) in matrix_iter {
///             println!("{:?} -> {}", idx, item);
///         }
///     }
///     Err(_) => {} // Shouldn't happen given these specific parameters
/// }
/// ```
/// This was added due to clippy warnings
type ConcatRetType<T> = Result<(Matrix<T>, Matrix<T>, Matrix<T>), MatrixError>;

pub fn concat<T>(lhs: Matrix<T>, rhs: Matrix<T>, axis: usize) -> ConcatRetType<T> where T: Clone + Default + Debug {
    if !check_concat_dims(lhs.shape(), rhs.shape(), axis) {
        return Err(MatrixError::DimError);
    }
    let lhs_iter: MatrixIter<T> = MatrixIter {
        mat: &lhs,
        index: vec![0; lhs.shape().len()],
        current_el: None,
        empty: false,
    };
    let rhs_iter: MatrixIter<T> = MatrixIter {
        mat: &rhs,
        index: vec![0; rhs.shape().len()],
        current_el: None,
        empty: false,
    };
    // Here unwrap is used since the same check that gets run on calc_concat_shape()
    // got run above so if it returns false the code wouldn't get to here.
    let f_shape = calc_concat_shape(lhs.shape(), rhs.shape(), axis).unwrap();
    let mut f_matrix: Matrix<T> = Matrix::new(f_shape, Layout::RowMajor);

    for (item, idx) in lhs_iter {
        match f_matrix.set(&idx, item) {
            Ok(_) => {},
            Err(err) => {
                return Err(err);
            }
        }
    }

    for (item, mut idx) in rhs_iter {
        idx[axis] += lhs.shape()[axis];
        match f_matrix.set(&idx, item) {
            Ok(_) => {},
            Err(err) => {
                return Err(err);
            }
        }
    }
    Ok((f_matrix, lhs, rhs))
}

/// Given two matrices the function first checks if they're broadcastable. The broadcast function
/// takes care of any dimensional issues. After we have broadcasted the matrices we then iterate
/// through them and subtract the elements. Again we subtract rhs from lhs so be careful how you
/// input them to the function.
///
/// The method takes ownership of rhs and lhs for it's duration and then returns it. In the future
/// we'll most likely add a feature to take them by reference.
/// # Examples
/// ```
/// use Cryptonic::tensor_library::layout::Layout;
/// use Cryptonic::tensor_library::matrix::{Matrix, MatrixIter, subtract};
/// let mut rhs: Matrix<i32> = Matrix::from_iter(vec![2, 2], 1..,Layout::RowMajor);
/// let mut lhs: Matrix<i32> = Matrix::from_iter(vec![2, 2], 1..,Layout::RowMajor);
///
/// let mut x = subtract(rhs, lhs);
/// match x {
///     Ok((val, _rhs, _lhs)) => {
///         let mut matrix_iter = MatrixIter {
///             mat: &val,
///             index: vec![0; val.shape().len()],
///             current_el: None,
///             empty: false,
///         };
///         for (item, idx) in matrix_iter {
///             println!("{:?} -> {}", idx, item); // Should return all zeroes
///         }
///     }
///     Err(_) => {} // Shouldn't happen given these specific parameters
/// }
/// ```
/// This was added due to clippy warnings
type SubRetType<T> = Result<(Matrix<T>, Matrix<T>, Matrix<T>), MatrixError>;

pub fn subtract<T>(mut lhs: Matrix<T>,mut rhs: Matrix<T>) -> SubRetType<T> where T: Clone + Default + Sub + Sub<Output = T>, <T as Sub>::Output: Clone + Default{
    let mut final_shape: Vec<usize> = vec![];
    match broadcast(lhs.shape(), lhs.layout, rhs.shape(), rhs.layout) {
        Ok((_shape, _lhs_strides, _rhs_strides)) => {
            lhs.set_shape(&_shape);
            rhs.set_shape(&_shape);
            lhs.set_strides(&_lhs_strides);
            rhs.set_strides(&_rhs_strides);
            final_shape = _shape;
        },
        Err(err) => {
            return Err(err);
        }
    }

    let lhs_iter: MatrixIter<T> = MatrixIter {
        mat: &lhs,
        index: vec![0; lhs.shape().len()],
        current_el: None,
        empty: false,
    };
    let rhs_iter: MatrixIter<T> = MatrixIter {
        mat: &rhs,
        index: vec![0; rhs.shape().len()],
        current_el: None,
        empty: false,
    };

    let mut new_matrix = Matrix::new(final_shape, Layout::RowMajor);

    for ((lhs_item, lhs_index), (rhs_item, rhs_index)) in lhs_iter.zip(rhs_iter){
        assert_eq!(lhs_index, rhs_index);
        match new_matrix.set(&lhs_index, lhs_item - rhs_item) {
            Ok(_) => {},
            Err(err) => {
                return Err(err);
            }
        }
    }
    Ok((new_matrix, lhs, rhs))
}


/// Given two matrices the function first checks if they're broadcastable. The broadcast function
/// takes care of any dimensional issues. After we have broadcasted the matrices we then iterate
/// through them and add the elements.
///
/// The method takes ownership of rhs and lhs for it's duration and then returns it. In the future
/// we'll most likely add a feature to take them by reference.
/// # Examples
/// ```
/// use Cryptonic::tensor_library::layout::Layout;
/// use Cryptonic::tensor_library::matrix::{add, Matrix, MatrixIter};
/// let mut rhs: Matrix<i32> = Matrix::from_iter(vec![2, 2], 1..,Layout::RowMajor);
/// let mut lhs: Matrix<i32> = Matrix::from_iter(vec![2, 2], 1..,Layout::RowMajor);
///
/// let mut x = add(rhs, lhs);
/// match x {
///     Ok((val, _rhs, _lhs)) => {
///         let mut matrix_iter = MatrixIter {
///             mat: &val,
///             index: vec![0; val.shape().len()],
///             current_el: None,
///             empty: false,
///         };
///         for (item, idx) in matrix_iter {
///             println!("{:?} -> {}", idx, item); // Should return all 2, 4, 6, etc.
///         }
///     }
///     Err(_) => {} // Shouldn't happen given these specific parameters
/// }
/// ```
/// This was added due to clippy warnings
type AddRetType<T> = Result<(Matrix<T>, Matrix<T>, Matrix<T>), MatrixError>;

pub fn add<T>(mut lhs: Matrix<T>,mut rhs: Matrix<T>) -> AddRetType<T> where T: Clone + Default + Add + Add<Output = T>, <T as Add>::Output: Clone + Default{
    let mut final_shape: Vec<usize> = vec![];
    match broadcast(lhs.shape(), lhs.layout, rhs.shape(), rhs.layout) {
        Ok((_shape, _lhs_strides, _rhs_strides)) => {
            lhs.set_shape(&_shape);
            rhs.set_shape(&_shape);
            lhs.set_strides(&_lhs_strides);
            rhs.set_strides(&_rhs_strides);
            final_shape = _shape;
        },
        Err(err) => {
            return Err(err);
        }
    }

    let lhs_iter: MatrixIter<T> = MatrixIter {
        mat: &lhs,
        index: vec![0; lhs.shape().len()],
        current_el: None,
        empty: false,
    };
    let rhs_iter: MatrixIter<T> = MatrixIter {
        mat: &rhs,
        index: vec![0; rhs.shape().len()],
        current_el: None,
        empty: false,
    };

    let mut new_matrix = Matrix::new(final_shape, Layout::RowMajor);

    for ((lhs_item, lhs_index), (rhs_item, rhs_index)) in lhs_iter.zip(rhs_iter){
        assert_eq!(lhs_index, rhs_index);
        match new_matrix.set(&lhs_index, lhs_item + rhs_item) {
            Ok(_) => {},
            Err(err) => {
                return Err(err);
            }
        }
    }
    Ok((new_matrix, lhs, rhs))
}


/// Given a matrix and a scalar we return the result of the multiplication of every element of the
/// matrix with the scalar.
///
/// In the future we might create a specific type: Scalar. To preserve backwards compatibility we'll
/// most likely give the function a different method and use the new function internally.
///
/// The method takes ownership of rhs and lhs for it's duration and then returns it. In the future
/// we'll most likely add a feature to take them by reference.
/// # Examples
///
/// ```
/// use Cryptonic::tensor_library::layout::Layout;
/// use Cryptonic::tensor_library::matrix::{Matrix, MatrixIter, multiply_scalar};
/// let mut lhs: Matrix<i32> = Matrix::from_iter(vec![2, 2], 1..,Layout::RowMajor);
/// let mut lhs_dup: Matrix<i32> = Matrix::from_iter(vec![2, 2], 1..,Layout::RowMajor);
/// let mut x = multiply_scalar(lhs, 5);
///
///     let mut matrix_iter = MatrixIter {
///         mat: &x,
///         index: vec![0; x.shape().len()],
///         current_el: None,
///         empty: false,
///     };
///
///     let mut matrix_iter_dup = MatrixIter {
///         mat: &lhs_dup,
///         index: vec![0; x.shape().len()],
///         current_el: None,
///         empty: false,
///     };
///
///     for ((item, idx), (item_2, idx_2)) in matrix_iter.zip(matrix_iter_dup) {
///         assert_eq!(item, item_2 * 5) // Should return all 5, 10, 15, etc.
///     }
/// ```
///
pub fn multiply_scalar<T>(mut lhs: Matrix<T>, rhs: T) -> Matrix<T> where T: Clone + Default + Mul + Mul<Output = T> + MulAssign, <T as Mul>::Output: Clone + Default{
    for i in 0..lhs.data.len(){
        lhs.data[i] = lhs.data[i].clone() * rhs.clone();
    }
    lhs
}

pub fn multiply_scalar_generic<T>(mut lhs: Matrix<i32>, rhs: T) -> Matrix<T> where T: Clone + Default + Mul<i32, Output = T>, i32: Mul<T>{
    let mut new_matrix: Matrix<T> = Matrix::new(lhs.shape().clone(), lhs.layout.clone());
    for i in 0..lhs.data.len(){
        new_matrix.data[i] = rhs.clone() * lhs.data[i].clone();
    }
    new_matrix
}


/// Given a matrix and a scalar we return the result of the multiplication of every element of the
/// matrix with the  of a different type.
pub fn multiply_scalar_diff_type<T>(mut lhs: Matrix<T>, rhs: i32) -> Matrix<T> where T: Clone + Default + Mul<i32, Output = T>{
    for i in 0..lhs.data.len(){
        lhs.data[i] = lhs.data[i].clone() * rhs.clone();
    }
    lhs
}

/*
/// Given a two matrices we return the result of the multiplication of them. The API user should be
/// careful of the fact that we change the shape and strides of the given matrices so if he needs
/// them to be a different shape he should reshape them after the operation.
///
/// The method takes ownership of rhs and lhs for it's duration and then returns it. In the future
/// we'll most likely add a feature to take them by reference.
/// # Examples
///
///
// use Cryptonic::{layout::Layout, matrix::*};
// let mut lhs: Matrix<i32> = Matrix::from_iter(vec![2, 2], 1..,Layout::RowMajor);
// let mut lhs_dup: Matrix<i32> = Matrix::from_iter(vec![2, 2], 1..,Layout::RowMajor);
// let mut x = multiply_scalar(lhs, 5);
//
//     let mut matrix_iter = MatrixIter {
//         mat: &x,
//         index: vec![0; x.shape().len()],
//         current_el: None,
//         empty: false,
//     };
//
//     let mut matrix_iter_dup = MatrixIter {
//         mat: &lhs_dup,
//         index: vec![0; x.shape().len()],
//         current_el: None,
//         empty: false,
//     };
//
//     for ((item, idx), (item_2, idx_2)) in matrix_iter.zip(matrix_iter_dup) {
//         assert_eq!(item, item_2 * 5) // Should return all 5, 10, 15, etc.
//     }
///
///
pub fn multiply<T>(mut lhs: Matrix<T>, mut rhs: Matrix<T>) -> Result<(Matrix<T>, Matrix<T>, Matrix<T>), MatrixError> where T: Clone + Default + Mul + Mul<Output = T> + MulAssign, <T as Mul>::Output: Clone + Default{
    let mut final_shape: Vec<usize> = vec![];
    match broadcast(lhs.shape(), lhs.layout, rhs.shape(), rhs.layout) {
        Ok((_shape, _lhs_strides, _rhs_strides)) => {
            lhs.set_shape(&_shape);
            rhs.set_shape(&_shape);
            lhs.set_strides(&_lhs_strides);
            rhs.set_strides(&_rhs_strides);
            final_shape = _shape;
        },
        Err(err) => {
            return Err(err);
        }
    }
    /*
    let lhs_iter: MatrixIter<T> = MatrixIter {
        mat: &lhs,
        index: vec![0; lhs.shape().len()],
        current_el: None,
        empty: false,
    };
    let rhs_iter: MatrixIter<T> = MatrixIter {
        mat: &rhs,
        index: vec![0; rhs.shape().len()],
        current_el: None,
        empty: false,
    };
    */
    let mut new_matrix = Matrix::new(final_shape, Layout::RowMajor);

    /*
    for ((lhs_item, lhs_index), (rhs_item, rhs_index)) in lhs_iter.zip(rhs_iter){
        assert_eq!(lhs_index, rhs_index);
        match new_matrix.set(&lhs_index, lhs_item + rhs_item) {
            Ok(_) => {},
            Err(err) => {
                return Err(err);
            }
        }
    }
    */
    Ok((new_matrix, lhs, rhs))
}
*/



/// Given a two two-dimensional matrices we return the result of the multiplication of them.
/// The API user should be careful of the fact that we change the shape and strides of the given
/// matrices so if he needs them to be a different shape he should reshape them after the operation.
///
/// The method takes ownership of rhs and lhs for it's duration and then returns it. In the future
/// we'll most likely add a feature to take them by reference.
/// # Examples
///
/// ```
/// use Cryptonic::tensor_library::layout::Layout;
/// use Cryptonic::tensor_library::matrix::{Matrix, MatrixIter, multiply_2d};
/// let mut mat1 = Matrix::from_iter(vec![2, 2], vec![1, 2, 3, 4], Layout::RowMajor);
///
/// let mut mat2 = Matrix::from_iter(vec![2, 2], vec![5, 6, 0, 7], Layout::RowMajor);
///
/// let (matmul, mat1, mat2) = multiply_2d(mat1, mat2).unwrap();
///
/// let mut matrix_iter = MatrixIter {
/// mat: &matmul,
/// index: vec![0; matmul.shape().len()],
/// current_el: None,
/// empty: false,
/// };
///
/// for (item, idx) in matrix_iter {
/// // Should print:
/// // [0, 0] -> 5
/// // [0, 1] -> 20
/// // [1, 0] -> 15
/// // [1, 1] -> 46
///     println!("{idx:?} -> {item}");
/// }
/// ```
/// This was added due to clippy warnings
type MulRetType2D<T> = Result<(Matrix<T>, Matrix<T>, Matrix<T>), MatrixError>;

pub fn multiply_2d<T>(mut lhs: Matrix<T>, mut rhs: Matrix<T>) -> MulRetType2D<T> where T: Display + Clone + Default + Mul + Mul<Output = T> + MulAssign + AddAssign, <T as Mul>::Output: Clone + Default{
    let mut final_shape: Vec<usize> = Vec::new();

    if lhs.shape.len() != 2 || rhs.shape.len() != 2 {
        return Err(MatrixError::MatmulShapeError);
    }
    match broadcast(lhs.shape(), lhs.layout, rhs.shape(), rhs.layout) {
        Ok((_shape, _lhs_strides, _rhs_strides)) => {
            lhs.set_shape(&_shape);
            rhs.set_shape(&_shape);
            lhs.set_strides(&_lhs_strides);
            rhs.set_strides(&_rhs_strides);
            final_shape = _shape;
        },
        Err(err) => {
            return Err(err);
        }
    }
    let mut new_matrix = Matrix::new(final_shape.clone(), Layout::RowMajor);
    let mut curr_sum = T::default();
    for i in 0..final_shape[0] {
        for j in 0..final_shape[1]{
            curr_sum = T::default();
            for l in 0..final_shape[1]{
                curr_sum += lhs.get_copy(&vec![i, l]).unwrap() * rhs.get_copy(&vec![l, j]).unwrap();
            }

            match new_matrix.set(&vec![i, j], curr_sum){
                Ok(_) => {},
                Err(err) => {
                    return Err(err);
                }
            }

        }
    }
    Ok((new_matrix, lhs, rhs))
}

/// Given a two one-dimensional matrices(i.e. vectors) we return the result of the dot product..
///
/// The method takes ownership of rhs and lhs for it's duration and then returns it. In the future
/// we'll most likely add a feature to take them by reference.
/// # Examples
///
/// ```
/// use Cryptonic::tensor_library::layout::Layout;
/// use Cryptonic::tensor_library::matrix::{Matrix, multiply_1d};
/// let mut mat1 = Matrix::from_iter(vec![4], vec![1, 2, 3, 4], Layout::RowMajor);
///
/// let mut mat2 = Matrix::from_iter(vec![4], vec![5, 6, 0, 7], Layout::RowMajor);
///
/// let (matmul, _mat1, _mat2) = multiply_1d(mat1, mat2).unwrap();
///
///     println!("{}", matmul); // Should print 45
/// ```
/// This was added due to clippy warnings
type MulRetType1D<T> = Result<(T, Matrix<T>, Matrix<T>), MatrixError>;
pub fn multiply_1d<T>(lhs: Matrix<T>, rhs: Matrix<T>) -> MulRetType1D<T> where T: Display + Clone + Default + Mul + Mul<Output = T> + MulAssign + AddAssign, <T as Mul>::Output: Clone + Default{
    if lhs.shape.len() != 1 || rhs.shape.len() != 1 && lhs.shape[0] == rhs.shape[0]{
        return Err(MatrixError::MatmulShapeError);
    }
    let mut curr_sum: T = T::default();
    for i in 0..lhs.shape()[0] {
        curr_sum += lhs.get_copy(&vec![i]).unwrap() * lhs.get_copy(&vec![i]).unwrap();
    }
    Ok((curr_sum, lhs, rhs))
}

/*
/// Given a two one-dimensional matrices(i.e. vectors) we return the result of the dot product..
///
/// The method takes ownership of rhs and lhs for it's duration and then returns it. In the future
/// we'll most likely add a feature to take them by reference.
/// # Examples
///
/// ```
/// use Cryptonic::tensor_library::layout::Layout;
/// use Cryptonic::tensor_library::matrix::{Matrix, multiply_1d};
/// let mut mat1 = Matrix::from_iter(vec![4], vec![1, 2, 3, 4], Layout::RowMajor);
///
/// let mut mat2 = Matrix::from_iter(vec![4], vec![5, 6, 0, 7], Layout::RowMajor);
///
/// let (matmul, _mat1, _mat2) = multiply_1d(mat1, mat2).unwrap();
///
///     println!("{}", matmul); // Should print 45
/// ```
/// This was added due to clippy warnings
type MulByRetType1D<T> = Result<(Matrix<T>, Matrix<T>, Matrix<i32>), MatrixError>;
pub fn multiplyby_1d<T>(lhs: Matrix<T>, rhs: Matrix<i32>) -> MulByRetType1D<T> where T: Display + Clone + Default + Mul + Mul<Output = T> + MulAssign + AddAssign, <T as Mul>::Output: Clone + Default{
    if rhs.shape.len() != 1 {
        return Err(MatrixError::MatmulShapeError);
    }
    //let mut ret_matrix = Matrix::new(lhs.shape().clone(), Layout::RowMajor);

    for i in 0..rhs.shape() {
        let (ret_matrix, _lhs, rhs) = multiply_scalar(_lhs, rhs.get_copy(&vec![i]));
    }
    Ok((rhs.clone(), rhs.clone(), rhs))
}
*/


#[derive(Debug, Clone)]
pub struct MatrixIter<'a, T> where T: Clone + Default + 'static{
    pub mat: &'a Matrix<T>,
    pub index: Vec<usize>,
    pub current_el: Option<(T, Vec<usize>)>,
    pub empty: bool,
}

impl<T> Iterator for MatrixIter<'_, T> where T: Clone + Default{
    type Item = (T, Vec<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.mat.check_bounds(&self.index) {
            Ok(_) => {
                if !self.empty {
                    // index was bounds checked so any such related panics from self.mat.get()
                    // indicate a bug.
                    self.current_el = Some((self.mat.get_copy(&self.index).unwrap(), self.index.clone()));
                    //print!("{:?} -> ", self.index)
                }
                else {
                    return None;
                }
            },
            Err(_) => {
                return None;
            }
        }
        let dims = self.mat.shape();
        let mut i: i32 = (self.mat.shape().len() - 1) as i32;
        while i >= 0 {
            if self.index[i as usize] + 1 < dims[i as usize] {
                self.index[i as usize] += 1;
                break;
            } else {
                self.index[i as usize] = 0;
                if i == 0 {
                    self.empty = true;
                    break;
                }
                i -= 1;
            }
        }
        self.current_el.clone()
    }
}

 /*
impl<T> Iterator for Matrix<T>{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}



impl<T> IntoIterator for Matrix<T> where T: Clone + Default + 'static{
    type Item = (T, Vec<usize>);
    type IntoIter= MatrixIter<'static, T>;

    fn into_iter(self) -> Self::IntoIter {
        MatrixIter {
            mat: &self,
            index: vec![0; self.shape().len()],
            current_el: None,
            empty: false
        }
    }
}
 */