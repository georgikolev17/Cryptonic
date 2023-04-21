use std::fmt::format;
use std::ptr::null;
use ndarray::{array, Array, ArrayD, Ix1, IxDyn};
use crate::cryptography::ciphtxt::CipherTextType;
use crate::cryptography::key_gen::custom_gen_keys;
use crate::neural_network::dense_layer::DenseLayer;
use crate::neural_network::layer_trait::Layer;
use crate::neural_network::nnet::Nnet;


/*
DOC:
if A is true and B is true, then A or B is true.
if A is true and B is false, then A or B is true.
if A is false and B is true, then A or B is true.
if A is false and B is false, then A or B is false.
*/
pub fn and_gate_net(A: bool, B: bool){

    // Set up encrypted data
    let (rck, sk, pk) = custom_gen_keys();
    let in_1 = CipherTextType::new(pk.encrypt_radix(A as u64, 8), pk.clone(), sk.clone());
    let in_2 = CipherTextType::new(pk.encrypt_radix(B as u64, 8), pk.clone(), sk.clone());

    // Set up neural network
    let mut nn : Nnet<CipherTextType> = Nnet::new();

    // Initialise the layers
    let mut layer1 : DenseLayer<CipherTextType> = DenseLayer::new(vec![2]);
    let mut layer2 : DenseLayer<CipherTextType> = DenseLayer::new(vec![2]);

    // Add the initialised layers to the neural network
    nn.add_layer(Box::new(layer1));
    nn.add_layer(Box::new(layer2));

    let mut biases = vec![0];
    biases[0] = 0;
    _ = nn.initialise_biases(biases);

    // Initialise weights
    let weights = vec![array![1, 0, 0, 1]];
    _ = nn.initialise_weights(weights);

    // Test the forward method
    let input : Array<CipherTextType, IxDyn> = array![in_1, in_2].into_owned().into_dyn();
    let res =  &nn.forward(input.clone()).unwrap();
    let final_result_1 = rck.decrypt::<u64, tfhe::shortint::ciphertext::KeyswitchBootstrap>(&res[0].CipherTxt);
    let final_result_2 = rck.decrypt::<u64, tfhe::shortint::ciphertext::KeyswitchBootstrap>(&res[1].CipherTxt);

    let mut frb_1 = false;
    let mut frb_2 = false;
    if final_result_1 == 1 {
        frb_1 = true;
    }
    if final_result_2 == 1 {
        frb_2 = true;
    }

    println!("{:?}", frb_1 && frb_2);
}