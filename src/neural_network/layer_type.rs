use crate::neural_network::layer_trait::Layer;
use crate::neural_network::test_layer::TestLayer;
use crate::tensor_library::matrix::Matrix;

pub enum LayerType<T> {
    // When a layer is implemented, it will be added here
    TestLayer(TestLayer<T>)
}
impl<T> Layer for LayerType<T> {
    type CType = T;

    fn forward(&mut self, input : Matrix<T>)  -> &Matrix<Self::CType> where <Self as Layer>::CType: Clone + Default {
        match self {
            // When a layer is implemented, it will be added here
            LayerType::TestLayer(testLayer) => return testLayer.forward(input),
        }
    }

    fn get_input_shape(&self) -> &Vec<usize> {
        match self {
            // When a layer is implemented, it will be added here
            LayerType::TestLayer(testLayer) => return testLayer.get_input_shape(),
        }
    }

    fn get_output_shape(&self) -> &Vec<usize> {
        match self {
            // When a layer is implemented, it will be added here
            LayerType::TestLayer(testLayer) => return testLayer.get_output_shape(),
        }
    }
}
