use crate::layout::Layout;
use super::{layout, errors};

pub struct Matrix<T> {
    pub shape: Vec<usize>,
    pub strides: Vec<usize>,
    pub data: Vec<T>,
    pub layout: Layout,
    pub size: usize
}

impl<T>  Matrix<T> {
    /// Constructs a new, non-empty Matrix<T> where cells are set to `T::default`.
    /// Use `Matrix::from_iter` if you want to set the matrix from an iterator.
    ///
    /// # Panics
    /// Panics if shape vector is empty
    /// Panics if data size provided through iterator isn't equal to the size defined by the shape
    ///
    /// # Example
    /// ```
    /// use crate::layout::Layout;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], layout: layout::Layout::RowMajor);
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
    /// use crate::layout::Layout;
    /// let mut mat: Matrix<i32> = Matrix::from_iter(vec![3, 4], 0..,layout: layout::Layout::RowMajor);
    /// ```
    pub fn from_iter(_shape: Vec<usize>, _data: impl IntoIterator<Item = T>, _layout: Layout) -> Matrix<T> {
        assert!(!_shape.is_empty());
        let _temp_shape = _shape.clone();
        Matrix {
            shape: _shape,
            strides: {
                let mut data_size: usize = 1;
                let mut strides: Vec<usize> = vec![0; _temp_shape.len()];

                if _layout == Layout::RowMajor {
                    for i in (1..(_temp_shape.len()+1)).rev() {
                        strides[i-1] = data_size;
                        data_size = strides[i-1] * _temp_shape[i-1];
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
                let data: Vec<_> = _data.into_iter().take(_temp_shape.iter().copied().reduce(|a, b| a*b).unwrap()).collect();
                assert_eq!(data.len(), _temp_shape.iter().copied().reduce(|a, b| a*b).unwrap());
                data
            },
            layout: _layout,
            size: _temp_shape.iter().copied().reduce(|a, b| a*b).unwrap()
        }
    }


    /// Returns the full size of a matrix
    ///
    /// # Examples
    /// ```
    /// use crate::layout::Layout;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], layout: layout::Layout::RowMajor);
    /// println!("{}", mat.size()); // Prints 12, because 12 = 3*4
    /// ```
    // TODO: Add tests
    pub fn size(&self) -> usize {
        self.shape.iter().copied().reduce(|a, b| a*b).unwrap()
    }


    /// Returns the shape of the matrix
    ///
    /// # Examples
    /// ```
    /// use crate::layout::Layout;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], layout: layout::Layout::RowMajor);
    /// println!("{:?}", mat.shape()); // Prints [3, 4]
    /// ```
    // TODO: Add tests
    pub fn shape(&self) -> &Vec<usize> {
        &self.shape
    }


    /// Returns the strides of the matrix
    /// This depends on the layout of the matrix(i.e. whether it's Row Major or Column Major)
    /// # Examples
    /// ```
    /// use crate::layout::Layout;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], layout: layout::Layout::RowMajor);
    /// println!("{:?}", mat.strides()); // Prints [4, 1]
    /// ```
    /// ```
    /// use crate::layout::Layout;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], layout: layout::Layout::ColumnMajor);
    /// println!("{:?}", mat.strides()); // Prints [1, 3]
    /// ```
    // TODO: Add tests
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
    /// use crate::layout::Layout;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], layout: layout::Layout::RowMajor);
    /// println!("{}", mat.check_bounds(vec![3, 4])); // Prints false because !3<3 && !4<4
    /// println!("{}", mat.check_bounds(vec![2, 3])); // Prints true because 2<3 && 3<4
    /// ```
    pub fn check_bounds(&self, idx: &Vec<usize>) -> bool{
        if idx.len() != self.shape.len() {
            return false;
        }
        let _size: usize = idx.len();
        for i in 0.._size{
            if idx[i] >= self.shape[i] {
                return false;
            }
        }
        return true;
    }

    /// Returns the physical id in the self.data vector
    /// Returns None if check_bounds is false
    ///
    /// # Examples
    /// ```
    /// use crate::layout::Layout;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], layout: layout::Layout::RowMajor);
    /// println!("{}", mat.get_physical_idx(vec![2, 1])); // Prints 9, because 9 = 2*4 + 1*1, since strides = [4, 1]
    /// ```
    // TODO: Add tests
    pub fn get_physical_idx(&self, idx: &Vec<usize>) -> Option<usize>{
        let mut return_val: usize = 0;
        if self.check_bounds(idx) {
            for i in 0..idx.len(){
                return_val += idx[i] * self.shape[i];
            }
            Some(return_val)
        } else {
            None
        }
    }

    /// Returns a non-mutable reference to a specific element in the matrix. On input an idx, we
    /// bound_check it and then return the element at that index.
    ///
    /// # Examples
    /// ```
    /// use crate::layout::Layout;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], layout: layout::Layout::RowMajor);
    /// println!("{}", mat.get(vec![2, 1])); // prints 0 because i32 default value is 0
    /// println!("{}", mat.get(vec![5, 6])); // prints None because self.get_physical_idx() fails
    /// ```
    // TODO: Add slicing
    // TODO: Add tests
    pub fn get(&self, idx: &Vec<usize>) -> Option<&T> {
        match self.get_physical_idx(idx) {
            None => None,
            Some(physical_idx) => {
                Some(&self.data[physical_idx])
            },
        }
    }

    /// Same as self.get(), but returns a mutable reference. Since the functionality is the same
    /// I haven't included examples
    ///
    // TODO: Add slicing
    // TODO: Add tests
    pub fn get_mut(&mut self, idx: &Vec<usize>) -> Option<&mut T> {
        match self.get_physical_idx(idx) {
            None => None,
            Some(physical_idx) => {
                Some(&mut self.data[physical_idx])
            },
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
    /// use crate::layout::Layout;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], layout: layout::Layout::RowMajor);
    /// mat.set(vec![0,0], 5);
    /// println!("{}", mat.get(vec![0,0])); // print 5
    /// ```
    // TODO: Add slicing
    // TODO: Add tests
    pub fn set(&mut self, idx: &Vec<usize>, value: T) -> bool {
        if let Some(cell) = self.get_mut(idx) {
            *cell = value;
            true
        } else {
            false
        }
    }

    // TODO: Once slices are added allow apply on specific slices
    // TODO: Add docs and examples
    pub fn apply<F: FnMut(&T)>(&self, mut func: F) {
        self.data.iter().for_each(|n| func(n));
    }

    /// Applies a specific function to every element of the matrix
    /// In the future we'll allow self.apply_mut on specific slices.
    ///
    /// # Examples
    /// ```
    /// use crate::layout::Layout;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![1, 2, 3], layout: layout::Layout);
    /// ```
    // TODO: Add docs and examples
    pub fn apply_mut<F: FnMut(&mut T)>(&mut self, mut func: F) {
        self.data.iter_mut().for_each(|n| func(n));
    }
}