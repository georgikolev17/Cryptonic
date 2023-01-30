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
    /// # Examples
    /// ```
    /// use crate::layout::Layout;
    /// let mut mat: Matrix<i32> = Matrix::new(vec![1, 2, 3], layout: layout::Layout);
    /// ```
    pub fn new(_shape: Vec<usize>, layout: Layout) -> Matrix<T>
        where
            T: Default,
    {
        Matrix::from_iter(_shape, (0..).map(|_| T::default()), layout)
    }

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

    // TODO: Add tests
    pub fn size(&self) -> usize {
        self.shape.iter().copied().reduce(|a, b| a*b).unwrap()
    }

    // TODO: Add tests
    pub fn shape(&self) -> &Vec<usize> {
        &self.shape
    }


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

    // TODO: Add slicing
    // TODO: Add tests
    // TODO: Possibly do this with Result<>
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

    // TODO: Add docs and examples
    pub fn apply_mut<F: FnMut(&mut T)>(&mut self, mut func: F) {
        self.data.iter_mut().for_each(|n| func(n));
    }
}