use crate::tensor_library::matrix::Matrix;

pub trait Layer<T> {
    type CType;

    fn forward(&mut self) -> Matrix<T>;

    fn get_input_shape(&self) -> Vec<usize>;

    fn get_output_shape(&self) -> Vec<usize>;
}