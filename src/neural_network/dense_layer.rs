use std::marker::PhantomData;
use crate::neural_network::layer_trait::Layer;
use ndarray::prelude::*;

/*
    In the case of the dense layer input shape will be equal to the size of the layer
    which is why the input shape is one-dimensional = Ix1

    This could l
*/

pub struct DenseLayer<T> {
    input_shape : Array<usize, Ix1>,
    output_shape : Array<usize, Ix1>,
    _phantom : PhantomData<T>
}

impl<T, D: Dimension> Layer<D> for DenseLayer<T> {
    type CType = T;

    fn forward(&mut self, input: Array<Self::CType, D>) -> Array<Self::CType, D> where <Self as Layer<D>>::CType: Clone + Default {
        input
    }

    fn get_input_shape(&self) -> &Array<usize, D> {
        &self.input_shape
    }

    fn get_output_shape(&self) -> &Array<usize, D> {
        &self.output_shape
    }
}

impl<T, D: Dimension> DenseLayer<T> {
    pub fn new(input_shape : Option<Array<usize, D>>, output_shape : Option<Array<usize, D>>) -> DenseLayer<T> {
        DenseLayer {
            input_shape : match input_shape {
                Some(shape) => shape,
                None => Vec::new()
            },
            output_shape : match output_shape {
                Some(shape) => shape,
                None => Vec::new()
            },
            _phantom : PhantomData::default(),
        }
    }
}