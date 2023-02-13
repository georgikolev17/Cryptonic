use crate::neural_network::layer_trait::Layer;
use crate::tensor_library::matrix::Matrix;

/// This is layer for testing the neural network
/// It is not for real use

pub struct TestLayer<T> {
    input_shape : Vec<usize>,
    output_shape : Vec<usize>,
}
impl<T> Layer for TestLayer<T> {
    type CType = T;

    fn forward(&mut self) -> &Matrix<Self::CType> where <Self as Layer>::CType: Clone + Default {
        todo!()
    }

    fn get_input_shape(&self) -> &Vec<usize> {
        &self.input_shape
    }

    fn get_output_shape(&self) -> &Vec<usize> {
        &self.output_shape
    }
}
