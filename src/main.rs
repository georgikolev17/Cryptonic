#![allow(non_snake_case)]

// Playground for encryption testing
// use concrete_integer::gen_keys_radix;
// use concrete_shortint::Parameters;
// use concrete_shortint::parameters::{CarryModulus, MessageModulus};
// use concrete_core::prelude::{DecompositionBaseLog, DecompositionLevelCount, GlweDimension, LweDimension, PolynomialSize, StandardDev};

use Cryptonic::nnet::Nnet;
use Cryptonic::layer::layer;

fn main() {
    let mut nnet : Nnet<layer> = Nnet::new();
    let l : layer = layer {};
    println!("{}", nnet.add_layer(&l, vec![1;5]));
    println!("{}", nnet.add_layer(&l, vec![1;5]));
    println!("{:?}", nnet.add_node(0, 1).unwrap());
}