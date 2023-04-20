use crate::neural_network::layer_trait::Layer;
use std::marker::PhantomData;
use ndarray::prelude::*;

/*
    In the case of the dense layer input shape will be equal to the size of the layer
    which is why the input shape is one-dimensional = Ix1

*/
#[derive(Clone)]
pub struct DenseLayer<T> {
    input_shape : Vec<usize>,
    output_shape : Vec<usize>,
    _phantom : PhantomData<T>,
    weights: Array<i32, Ix1>,
    bias: i32,
}

impl<T> Layer<T> for DenseLayer<T> where T : Copy {

    fn forward(&self, input: ArrayD<T>) -> ArrayD<T> {
        input
    }

    fn get_input_shape(&self) -> &Vec<usize> {
        &self.input_shape
    }

    fn get_output_shape(&self) -> &Vec<usize> {
        &self.output_shape
    }

    fn get_weights(&self) -> &Array<i32, Ix1> {
        &self.weights
    }

    fn get_bias(&self) -> i32 {
        self.bias
    }

    fn change_weights(&mut self, new_weights: Array<i32, Ix1>) {
        self.weights = new_weights;
    }

    fn change_bias(&mut self, new_bias: i32) {
        self.bias = new_bias;
    }
}

impl<T> DenseLayer<T> where T : Default {
    pub fn new(layer_shape : Vec<usize>) -> DenseLayer<T> {
        DenseLayer {
            input_shape : layer_shape.clone(),
            output_shape : layer_shape.clone(),
            _phantom : PhantomData::default(),
            weights : Array::default(0),
            bias : 0
        }
    }
}