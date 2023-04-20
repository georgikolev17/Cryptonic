use std::marker::PhantomData;
use ndarray::{Array, Dimension, Ix1};
use crate::neural_network::layer_trait::Layer;

/// This is layer for testing the neural network
/// It is not for real use

use ndarray::prelude::*;

pub struct TestLayer<T> {
    input_shape : Array<usize, Ix1>,
    output_shape : Array<usize, Ix1>,
    phantom: PhantomData<T>,
}
impl<T> Layer for TestLayer<T> {
    type CType = T;

    fn forward(&mut self, input : Array<T, Ix1>) -> Array<Self::CType, Ix1> where <Self as Layer>::CType: Clone + Default {
        todo!()
    }

    fn get_input_shape(&self) -> &Array<usize, Ix1> {
        &self.input_shape
    }

    fn get_output_shape(&self) -> &Array<usize, Ix1> {
        &self.output_shape
    }
}
