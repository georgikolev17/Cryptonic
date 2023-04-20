#![allow(non_snake_case)]

use Cryptonic::neural_network::dense_layer::DenseLayer;
use Cryptonic::neural_network::layer_type::LayerType;
use Cryptonic::neural_network::nnet::Nnet;

fn main() {
    let dense_layer1: DenseLayer<i32> = DenseLayer::new(Some(vec![2]), Some(vec![2]));
    let layer1 = LayerType::DenseLayer(dense_layer1);
    let weights1 : Vec<i32> = vec![2 ; 4];
    let biases1 : Vec<i32> = vec![0; 2];

    let dense_layer2 : DenseLayer<i32> = DenseLayer::new(Some(vec![2]), Some(vec![2]));
    let layer2 = LayerType::DenseLayer(dense_layer2);
    let biases2 : Vec<i32> = vec![1; 2];

    let mut network : Nnet<i32>  = Nnet::new();
    let id1 = network.add_layer(layer1, biases1);
    let id2 = network.add_layer(layer2, biases2);

    let _x = network.add_link(None, Some(id1), Vec::new());
    let _x = network.add_link(Some(id1), Some(id2), weights1);
    let _x = network.add_link(Some(id2), None, Vec::new());

    let input = vec![5, 5];
    let input_matrix : Matrix<i32> = Matrix::from_iter(vec![input.len()], input, Layout::RowMajor);

    let result = network.forward(input_matrix).unwrap();

    println!("{:?}", result);
}