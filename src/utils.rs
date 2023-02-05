use super::{errors::MatrixError, layout::Layout};


/// Calculates strides from a given shape and layout.
///
/// # Examples
/// ```
/// use Cryptonic::{matrix::Matrix, layout::Layout};
/// use Cryptonic::utils::calc_strides_from_shape;
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
/// use Cryptonic::utils::calc_size_from_shape;
/// let mut val = calc_size_from_shape(&vec![3, 4]);
/// println!("{}", val); // Prints 12, because 12 = 3*4
/// ```
/// TODO: Add tests
pub fn calc_size_from_shape(shape: &Vec<usize>) -> usize {
    shape.iter().copied().reduce(|a, b| a * b).unwrap()
}

/// Checks if the dimensions are concat-able. This is when they differ only by the dimension
/// specified by the axis parameter.
///
/// # Examples
/// ```
/// use Cryptonic::utils::calc_size_from_shape;
/// use Cryptonic::utils::check_concat_dims;
/// let vec_1 = vec![3, 4, 5];
/// let vec_2 = vec![3, 2, 5];
/// let mut val = check_concat_dims(&vec_1, &vec_2, 1);
/// println!("{:?}", val); // Prints true
/// ```
pub fn check_concat_dims(lhs: &Vec<usize>, rhs: &Vec<usize>, axis: usize) -> bool{
    if lhs.len() != rhs.len() {
        return false;
    }
    let len = lhs.len();

    for i in 0..len{
        if i == axis {
            continue;
        }
        if lhs[i] != rhs[i]{
            return false;
        }
    }
    true
}

/// Calculates the resulting shape of two concatenated matrices. What happens is that all of the
/// dimensions other than the one specified by the axis parameter stay the same.
/// Final_Matrix.shape()[axis] = lhs.shape()[axis]+rhs.shape()[axis]
///
/// # Examples
/// ```
/// use Cryptonic::utils::calc_size_from_shape;
/// use Cryptonic::utils::calc_concat_shape;
/// let vec_1 = vec![3, 4, 5];
/// let vec_2 = vec![3, 2, 5];
/// let mut val = calc_concat_shape(&vec_1, &vec_2, 1);
/// println!("{:?}", val); // Prints [3, 6, 5]
/// ```
pub fn calc_concat_shape(lhs: &Vec<usize>, rhs: &Vec<usize>, axis: usize) -> Option<Vec<usize>>{
    if !check_concat_dims(lhs, rhs, axis) {
        return None
    }
    let mut f_vec = lhs.clone();
    f_vec[axis] += rhs[axis];
    return Some(f_vec);
}