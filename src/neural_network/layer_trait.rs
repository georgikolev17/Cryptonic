use crate::tensor_library::matrix::Matrix;

pub trait Layer {
    type CType;

    fn forward(&mut self, input : Matrix<Self::CType>) -> Matrix<Self::CType> where <Self as Layer>::CType: Clone + Default;

    fn get_input_shape(&self) -> &Vec<usize>;

    fn get_output_shape(&self) -> &Vec<usize>;
}