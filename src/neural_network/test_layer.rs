use std::marker::PhantomData;
use crate::neural_network::layer_trait::Layer;

/// This is layer for testing the neural network
/// It is not for real use

pub struct TestLayer<T> {
    input_shape : Vec<usize>,
    output_shape : Vec<usize>,
    phantom: PhantomData<T>,
}
impl<T> Layer for TestLayer<T> {
    type CType = T;

    fn forward(&mut self, input : Matrix<T>) -> Matrix<Self::CType> where <Self as Layer>::CType: Clone + Default {
        todo!()
    }

    fn get_input_shape(&self) -> &Vec<usize> {
        &self.input_shape
    }

    fn get_output_shape(&self) -> &Vec<usize> {
        &self.output_shape
    }
}
