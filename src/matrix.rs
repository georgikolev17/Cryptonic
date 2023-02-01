use super::{errors::MatrixError, layout::Layout};

#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    pub shape: Vec<usize>,
    pub strides: Vec<usize>,
    pub data: Vec<T>,
    pub layout: Layout,
    pub size: usize,
}

impl<T> Matrix<T> {
    /// Constructs a new, non-empty Matrix<T> where cells are set to `T::default`.
    /// Use `Matrix::from_iter` if you want to set the matrix from an iterator.
    ///
    /// # Panics
    /// Panics if shape vector is empty
    /// Panics if data size provided through iterator isn't equal to the size defined by the shape
    ///
    /// # Example
    /// ```
    /// use Cryptonic::{matrix::Matrix, layout::Layout};
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
    /// let mut mat: Matrix<i32> = Matrix::from_iter(vec![3, 4], 0.., Layout::RowMajor);
    /// ```
    pub fn from_iter(
        _shape: Vec<usize>,
        _data: impl IntoIterator<Item = T>,
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
            size: _temp_shape.iter().copied().reduce(|a, b| a * b).unwrap(),
        }
    }

    /// Returns the full size of a matrix
    ///
    /// # Examples
    /// ```
    /// use Cryptonic::{matrix::Matrix, layout::Layout};
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
    /// use Cryptonic::{matrix::Matrix, layout::Layout};
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
    /// use Cryptonic::{matrix::Matrix, layout::Layout, errors::MatrixError};
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
    /// use Cryptonic::{matrix::Matrix, layout::Layout};
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::RowMajor);
    /// println!("{:?}", mat.strides()); // Prints [4, 1]
    /// ```
    /// ```
    /// use Cryptonic::{matrix::Matrix, layout::Layout};
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::ColumnMajor);
    /// println!("{:?}", mat.strides()); // Prints [1, 3]
    /// ```
    pub fn strides(&self) -> &Vec<usize> {
        &self.strides
    }

    /// This is a utilities function, which is used any time a index is given as an input.
    /// Takes a idx: Vec<usize> and checks that the number of dimensions is the same as the
    /// matrix. Then for each element in idx it's checked that it's smaller than its corresponding
    /// element in &self.shape.
    ///
    /// # Examples
    /// ```
    /// use Cryptonic::{matrix::Matrix, layout::Layout};
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4],Layout::RowMajor);
    /// println!("{:?}", mat.check_bounds(&vec![3, 4]).err()); // Prints Error because !3<3 && !4<4
    /// println!("{:?}", mat.check_bounds(&vec![2, 3]).unwrap()); // Prints () because 2<3 && 3<4
    /// ```
    pub fn check_bounds(&self, idx: &Vec<usize>) -> Result<(), MatrixError> {
        if idx.len() != self.shape.len() {
            return Err(MatrixError::DimError);
        }
        let _size: usize = idx.len();
        for i in 0.._size {
            if idx[i] >= self.shape[i] {
                return Err(MatrixError::OutOfBounds);
            }
        }
        return Ok(());
    }

    /// Returns the physical id in the self.data vector
    /// Returns None if check_bounds is false
    ///
    ///
    ///
    /// # Examples
    /// ```
    /// use Cryptonic::{matrix::Matrix, layout::Layout};
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::RowMajor);
    /// println!("{}", mat.get_physical_idx(&vec![2, 1]).unwrap()); // Prints 9, because 9 = 2*4 + 1*1, since strides = [4, 1]
    /// ```
    pub fn get_physical_idx(&self, idx: &Vec<usize>) -> Result<usize, MatrixError> {
        let mut return_val: usize = 0;
        match self.check_bounds(idx) {
            Ok(()) => {
                for i in 0..idx.len() {
                    return_val += idx[i] * self.strides[i];
                }
                Ok(return_val)
            }
            // This broadcasts the error from self.check_bounds()
            Err(err) => Err(err),
        }
    }

    /// Returns a non-mutable reference to a specific element in the matrix. On input an idx, we
    /// bound_check it and then return the element at that index.
    ///
    /// # Examples
    /// ```
    /// use Cryptonic::{matrix::Matrix, layout::Layout};
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
    /// use Cryptonic::{matrix::Matrix, layout::Layout};
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

    /// Apply a function to all cells of the matrix.
    /// Cells are provided as immutable references to the function,
    /// if you want to modify the cells, use `apply_mut`.
    ///
    /// # Examples
    /// ```
    /// // Get the sum of all cells
    /// use Cryptonic::{matrix::Matrix, layout::Layout};
    /// let mut mat: Matrix<i32> = Matrix::from_iter(vec![3, 6], 1..,Layout::RowMajor);
    /// let mut sum = 0;
    /// mat.apply(|n| sum += *n);
    ///
    /// assert_eq!(sum, 171);
    /// ```
    // TODO: Once slices are added allow apply on specific slices
    pub fn apply<F: FnMut(&T)>(&self, mut func: F) {
        self.data.iter().for_each(|n| func(n));
    }

    /// Apply a function to all cells of the matrix.
    /// Cells are provided as mutable references to the function,
    /// and can therefore be modified.
    ///
    /// # Examples
    /// ```
    /// // Modify all cells with a function
    /// use Cryptonic::{matrix::Matrix, layout::Layout};
    /// let mut mat: Matrix<i32> = Matrix::from_iter(vec![3, 4], 1..,Layout::RowMajor);
    /// mat.apply_mut(|mut n| *n *= 2);
    ///
    /// assert_eq!(mat.get(&vec![0, 0]).unwrap(), &2);
    /// assert_eq!(mat.get(&vec![0, 1]).unwrap(), &4);
    /// assert_eq!(mat.get(&vec![0, 2]).unwrap(), &6);
    /// ```
    /// TODO: Once slices are added allow apply on specific slices
    pub fn apply_mut<F: FnMut(&mut T)>(&mut self, mut func: F) {
        self.data.iter_mut().for_each(|n| func(n));
    }

    // Transposes a matrix
    pub fn transpose(&mut self) {
        self.shape.reverse();
        self.strides.reverse();
        match self.layout {
            Layout::RowMajor => self.layout = Layout::ColumnMajor,
            Layout::ColumnMajor => self.layout = Layout::RowMajor,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Calculates strides from a given shape and layout.
///
/// # Examples
/// ```
/// use Cryptonic::{matrix::Matrix, layout::Layout};
/// use Cryptonic::matrix::calc_strides_from_shape;
/// let mut val = calc_strides_from_shape(&vec![3, 4], Layout::RowMajor);
/// println!("{:?}", val); // Prints [4, 1]
/// ```
/// TODO: Add tests
pub fn calc_strides_from_shape(shape: &Vec<usize>, layout: Layout) -> Vec<usize> {
    let mut data_size: usize = 1;
    let mut strides: Vec<usize> = vec![0; shape.len()];

    if layout == Layout::RowMajor {
        for i in (1..(shape.len() + 1)).rev() {
            strides[i - 1] = data_size;
            data_size = strides[i - 1] * shape[i - 1];
        }
    }
    // For Column Major Layout
    else {
        for i in 0..shape.len() {
            strides[i] = data_size;
            data_size = strides[i] * shape[i];
        }
    }
    strides
}

/// Calculates strides from a given shape and layout.
///
/// # Examples
/// ```
/// use Cryptonic::matrix::calc_size_from_shape;
/// let mut val = calc_size_from_shape(&vec![3, 4]);
/// println!("{}", val); // Prints 12, because 12 = 3*4
/// ```
/// TODO: Add tests
pub fn calc_size_from_shape(shape: &Vec<usize>) -> usize {
    shape.iter().copied().reduce(|a, b| a * b).unwrap()
}

/// Given two matrices the function broadcast either makes their shapes compatible or returns
/// an error stating that the given matrices aren't broadcastable.
///
/// # Examples
/// ```
/// use Cryptonic::{matrix::broadcast, layout::Layout};
/// match broadcast(&vec![3, 4], Layout::RowMajor, &vec![7, 3, 4], Layout::RowMajor) {
///             Ok((v1, v2, v3)) => {
///                 println!("{:?}", v1); // Should print out [7, 3, 4]
///                 println!("{:?}", v2); // Should print out [0, 4, 1]
///                 println!("{:?}", v3); // Should print out [12, 4, 1]
///             },
///             Err(E) => panic!("{}", E)
///         }
/// ```
///
/// TODO: Add tests
pub fn broadcast(
    lhs_shape: &Vec<usize>,
    lhs_layout: Layout,
    rhs_shape: &Vec<usize>,
    rhs_layout: Layout,
) -> Result<(Vec<usize>, Vec<usize>, Vec<usize>), MatrixError> {
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
