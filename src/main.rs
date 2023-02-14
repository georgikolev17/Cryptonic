#![allow(non_snake_case)]

use Cryptonic::neural_network::dense_layer::DenseLayer;
use Cryptonic::neural_network::layer_type::LayerType;
use Cryptonic::neural_network::nnet::Nnet;
use Cryptonic::tensor_library::layout::Layout;
use Cryptonic::tensor_library::matrix::Matrix;

fn main() {
    let dense_layer1: DenseLayer<i32> = DenseLayer::new(Some(vec![2]), Some(vec![2]));
    let layer1 = LayerType::DenseLayer(dense_layer1);
    let weights1 : Vec<i32> = vec![2 ; 4];
    let biases1 : Vec<i32> = vec![17; 2];

    let dense_layer2 : DenseLayer<i32> = DenseLayer::new(Some(vec![2]), Some(vec![2]));
    let layer2 = LayerType::DenseLayer(dense_layer2);
    let biases2 : Vec<i32> = vec![0; 2];
    let weights2 : Vec<i32> = vec![1 ; 4];

    let mut network : Nnet<i32>  = Nnet::new();
    let id1 = network.add_layer(layer1, weights1, biases1);
    let id2 = network.add_layer(layer2, weights2, biases2);

    let _x = network.add_node(None, Some(id1));
    let _x = network.add_node(Some(id1), Some(id2));
    let _x = network.add_node(Some(id2), None);

    let input = vec![12, 42];
    let input_matrix : Matrix<i32> = Matrix::from_iter(vec![input.len()], input, Layout::RowMajor);

    let result = network.forward(input_matrix).unwrap();

    println!("{:?}", result);
}