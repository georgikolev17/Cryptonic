#![allow(non_snake_case)]

use Cryptonic::neural_network::dense_layer::DenseLayer;
use Cryptonic::neural_network::layer_type::LayerType;
use Cryptonic::neural_network::nnet::Nnet;
use Cryptonic::tensor_library::matrix::Matrix;

fn main() {
    let dense_layer1: DenseLayer<i32> = DenseLayer::new(Some(vec![2, 2]), Some(vec![2, 2]));
    let dense_layer2 : DenseLayer<i32> = DenseLayer::new(Some(vec![2, 2]), Some(vec![2, 2]));

    let mut network : Nnet<i32>  = Nnet::new();

    let layer_type = LayerType::DenseLayer(dense_layer1);
    network.add_layer(LayerType::DenseLayer(dense_layer1), )
}