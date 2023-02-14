use std::marker::PhantomData;
use crate::neural_network::layer_trait::Layer;
use crate::tensor_library::matrix::Matrix;

pub struct DenseLayer<T> {
    input_shape : Vec<usize>,
    output_shape : Vec<usize>,
    _phantom : PhantomData<T>
}

impl<T> Layer for DenseLayer<T> {
    type CType = T;

    fn forward(&mut self, input: Matrix<Self::CType>) -> &Matrix<Self::CType> where <Self as Layer>::CType: Clone + Default {
        &input
    }

    fn get_input_shape(&self) -> &Vec<usize> {
        &self.input_shape
    }

    fn get_output_shape(&self) -> &Vec<usize> {
        &self.output_shape
    }
}

impl<T> DenseLayer<T> {
    pub fn new(input_shape : Option<Vec<usize>>, output_shape : Option<Vec<usize>>) -> DenseLayer<T> {
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