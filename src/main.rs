#![allow(non_snake_case)]

use std::ops::{Mul, MulAssign};
use ndarray::{Array, array};
use Cryptonic::neural_network::dense_layer::DenseLayer;
use Cryptonic::neural_network::nnet::Nnet;


fn main() {
    let mut arr1 = array![2, 2, 2];
    let arr2 = array![3, 3, 3];
    arr1.mul_assign(&arr2);

    println!("{}", arr1);
    println!("{}", arr1+5);
}