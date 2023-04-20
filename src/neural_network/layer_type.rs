use ndarray::prelude::*;

use crate::neural_network::dense_layer::DenseLayer;
use crate::neural_network::layer_trait::Layer;


pub enum LayerType<T> {
    // When a layer is implemented, it will be added here
    DenseLayer(DenseLayer<T>),
}
impl<T, D: Dimension> Layer<D> for LayerType<T> {
    type CType = T;

    fn forward(&mut self, input : Array<Self::CType, D>)  -> Array<Self::CType, D> where <Self as Layer<D>>::CType: Clone + Default {
        match self {
            // When a layer is implemented, it will be added here
            LayerType::DenseLayer(dense_layer) => return dense_layer.forward(input),
        }
    }

    fn get_input_shape(&self) -> &Array<usize, D> {
        match self {
            // When a layer is implemented, it will be added here
            LayerType::DenseLayer(dense_layer) => return dense_layer.get_input_shape(),
        }
    }

    fn get_output_shape(&self) -> &Array<usize, D> {
        match self {
            // When a layer is implemented, it will be added here
            LayerType::DenseLayer(dense_layer) => return dense_layer.get_output_shape(),
        }
    }
}
