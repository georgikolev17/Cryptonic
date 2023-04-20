use crate::neural_network::layer_trait::Layer;
use std::marker::PhantomData;
use ndarray::{IxDynImpl, OwnedRepr};
use ndarray::prelude::*;

/*
    In the case of the dense layer input shape will be equal to the size of the layer
    which is why the input shape is one-dimensional = Ix1

*/

pub struct DenseLayer<T> {
    input_shape : Vec<usize>,
    output_shape : Vec<usize>,
    _phantom : PhantomData<T>,
    weights: Array<i16, Ix1>,
    bias: i16,
}

impl<T: ndarray::RawData> Layer for DenseLayer<T> {
    type CType = T;

    fn forward(&mut self, input: ArrayD<Self::CType>) -> Array<Self::CType, Dim<IxDynImpl>> where <Self as Layer>::CType: Clone + Default {
        input
    }

    fn get_input_shape(&self) -> &Vec<usize> {
        &self.input_shape
    }

    fn get_output_shape(&self) -> &Vec<usize> {
        &self.output_shape
    }

    fn get_weights(&self) -> &Array<i16, Ix1> {
        &self.weights
    }

    fn get_bias(&self) -> i16 {
        self.bias
    }

    fn change_weights(&mut self, new_weights: Array<i16, Ix1>) {
        for i in 0..(self.weights.len()-1) {
            self.weights[i] = new_weights[i];
        }
    }

    fn change_bias(&mut self, new_bias: i16) {
        self.bias = new_bias;
    }
}

impl<T> DenseLayer<T> {
    pub fn new(layer_shape : Vec<usize>) -> DenseLayer<T> {
        DenseLayer {
            input_shape : layer_shape.clone(),
            output_shape : layer_shape.clone(),
            _phantom : PhantomData::default(),
            weights : Array::zeros(20),
            bias :
        }
    }
}