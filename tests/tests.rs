extern crate core;

#[cfg(test)]
mod test_cryptography{
    use std::time::Instant;
    use Cryptonic::cryptography::key_gen::custom_gen_keys;
    use Cryptonic::cryptography::ciphtxt;
    use Cryptonic::cryptography::ciphtxt::CipherTextType;


    #[test]
    fn test_key_gen(){
        // If throws than there is a problem.
        // We also log the time it takes for the keygen

        let now = Instant::now();

        let (rck, sk, pk) = custom_gen_keys();
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }

    #[test]
    fn test_cphtxttype_new_default() {
        let (rck, sk, pk) = custom_gen_keys();
        let ciphertxt = CipherTextType::new(pk.encrypt_radix(10u64, 8), pk, sk);
        let defaultedcphtxt = CipherTextType::default();
        assert!(defaultedcphtxt.is_def())
    }

    #[test]
    fn test_ciphertext_add(){
        let (rck, sk, pk) = custom_gen_keys();
        let ct_1 = CipherTextType::new(pk.encrypt_radix(10u64, 8), pk.clone(), sk.clone());
        let ct_2 = CipherTextType::new(pk.encrypt_radix(15u64, 8), pk, sk);

        let ct_3 = ct_1 + ct_2;

        assert!(!ct_3.is_def());
        assert_eq!(rck.decrypt::<u64, tfhe::shortint::ciphertext::KeyswitchBootstrap>(&ct_3.CipherTxt), 25u64);
    }

    #[test]
    fn test_ciphertext_mul(){
        let (rck, sk, pk) = custom_gen_keys();
        let ct_1 = CipherTextType::new(pk.encrypt_radix(202u64, 8), pk.clone(), sk.clone());
        let ct_2 = CipherTextType::new(pk.encrypt_radix(10u64, 8), pk, sk);

        let ct_3 = ct_1 * ct_2;

        assert!(!ct_3.is_def());
        assert_eq!(rck.decrypt::<u64, tfhe::shortint::ciphertext::KeyswitchBootstrap>(&ct_3.CipherTxt), 2020u64);
    }

    #[test]
    fn test_ciphertext_add_i32(){
        let (rck, sk, pk) = custom_gen_keys();
        let ct_1 = CipherTextType::new(pk.encrypt_radix(202u64, 8), pk.clone(), sk.clone());

        let ct_3 = ct_1 + 50i32;

        assert!(!ct_3.is_def());
        assert_eq!(rck.decrypt::<u64, tfhe::shortint::ciphertext::KeyswitchBootstrap>(&ct_3.CipherTxt), 252u64);
    }

    #[test]
    fn test_ciphertext_mul_i32(){
        let (rck, sk, pk) = custom_gen_keys();
        let ct_1 = CipherTextType::new(pk.encrypt_radix(202u64, 8), pk.clone(), sk.clone());

        let ct_3 = ct_1 * 50i32;

        assert!(!ct_3.is_def());
        assert_eq!(rck.decrypt::<u64, tfhe::shortint::ciphertext::KeyswitchBootstrap>(&ct_3.CipherTxt), 10100u64);
    }
}

#[cfg(test)]
mod test_layers {
    use ndarray::array;
    use Cryptonic::neural_network::dense_layer::DenseLayer;
    use Cryptonic::neural_network::layer_trait::Layer;
    use Cryptonic::neural_network::nnet::Nnet;

    #[test]
    fn test_layer_shape() {
        let layer : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);
        let input_shape : Vec<usize> = vec![3, 2];
        assert_eq!(layer.get_input_shape(), &input_shape);
    }
    #[test]
    fn test_change_bias() {
        let mut layer : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);
        layer.change_bias(20);
        assert_eq!(layer.get_bias(), 20);
    }
    #[test]
    fn test_change_weights() {
        let mut layer : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);
        layer.change_weights(array![10, 10, 10]);
        assert_eq!(layer.get_weights(), array![10, 10, 10]);
    }
}

#[cfg(test)]
mod test_neural_network {
    use std::fmt::format;
    use std::ptr::null;
    use ndarray::{array, Array, ArrayD, Ix1, IxDyn};
    use Cryptonic::neural_network::dense_layer::DenseLayer;
    use Cryptonic::neural_network::layer_trait::Layer;
    use Cryptonic::neural_network::nnet::Nnet;

    #[test]
    fn test_adding_a_new_layer() {
        let mut nn : Nnet<i32> = Nnet::new();
        let mut layer : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);
        nn.add_layer(Box::new(layer));
        assert_eq!(nn.layers.len(), 1);
    }

    #[test]
    fn test_should_throw_an_error_if_number_of_weights_is_inappropriate() {
        let mut nn : Nnet<i32> = Nnet::new();

        let mut layer1 : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);
        let mut layer2 : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);

        nn.add_layer(Box::new(layer1));
        nn.add_layer(Box::new(layer2));

        let weights = vec![array![3, 3, 3], array![3, 3, 3]];

        let expected_res: Result<(), String> =  Err(("1 sets of weights expected. Received 2!").to_string());
        assert_eq!(nn.initialise_weights(weights), expected_res);
        assert!(!nn.are_weights_initialised);
    }

    #[test]
    fn should_initialise_weights_if_they_are_in_correct_format() {
        let mut nn : Nnet<i32> = Nnet::new();

        let mut layer1 : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);
        let mut layer2 : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);

        nn.add_layer(Box::new(layer1));
        nn.add_layer(Box::new(layer2));

        let weights = vec![array![3, 3, 3]];

        _ = nn.initialise_weights(weights);

        assert_eq!(nn.layers[0].get_weights(), array![3, 3, 3]);
        assert!(nn.are_weights_initialised);
    }

    #[test]
    fn should_throw_an_error_if_number_of_biases_is_incorrect() {
        let mut nn : Nnet<i32> = Nnet::new();

        let mut layer1 : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);
        let mut layer2 : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);

        nn.add_layer(Box::new(layer1));
        nn.add_layer(Box::new(layer2));

        let biases = vec![1, 2];

        let expected_res = Err(("1 biases expected. Received 2!").to_string());

        assert_eq!(nn.initialise_biases(biases), expected_res);
        assert!(!nn.are_biases_initialised);
    }

    #[test]
    fn test_initialising_biases() {
        let mut nn : Nnet<i32> = Nnet::new();

        let mut layer1 : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);
        let mut layer2 : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);

        nn.add_layer(Box::new(layer1));
        nn.add_layer(Box::new(layer2));

        let biases = vec![1];

        _ = nn.initialise_biases(biases);

        assert!(nn.are_biases_initialised);
        assert_eq!(nn.layers[0].get_bias(), 1);
    }

    #[test]
    fn forward_should_throw_an_error_if_weights_are_not_initialised() {
        let mut nn : Nnet<i32> = Nnet::new();

        let mut layer1 : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);
        let mut layer2 : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);

        nn.add_layer(Box::new(layer1));
        nn.add_layer(Box::new(layer2));

        let biases = vec![1];

        _ = nn.initialise_biases(biases);

        let expected_res = Err("You have not initialised the weights for this neural network!");

        let input : Array<i32, IxDyn> = array![1, 2, 3, 4].into_owned().into_dyn();
        assert_eq!(nn.forward(input), expected_res);
    }

    #[test]
    fn forward_test() {
        // Initialise the neural network
        let mut nn : Nnet<i32> = Nnet::new();

        // Initialise the layers
        let mut layer1 : DenseLayer<i32> = DenseLayer::new(vec![3]);
        let mut layer2 : DenseLayer<i32> = DenseLayer::new(vec![2]);

        // Add the initialised layers to the neural network
        nn.add_layer(Box::new(layer1));
        nn.add_layer(Box::new(layer2));

        // Initialise biases
        let biases = vec![0];
        _ = nn.initialise_biases(biases);

        // Initialise weights
        let weights = vec![array![5, 10, 3, 12, 7, 4]];
        _ = nn.initialise_weights(weights);

        // Test the forward method
        let input : Array<i32, IxDyn> = array![1, 5, 10].into_owned().into_dyn();
        let expected_res : ArrayD<i32> = array![85, 87].into_dyn();
        assert_eq!(nn.forward(input).unwrap(), expected_res);
    }
}

#[cfg(test)]
mod test_activation{
    use Cryptonic::cryptography::ciphtxt::CipherTextType;
    use Cryptonic::cryptography::key_gen::custom_gen_keys;
    use Cryptonic::cryptography::activations::{relu, binary_step};

    #[test]
    fn test_relu(){
        let (rck, sk, pk) = custom_gen_keys();
        let ct_1 = CipherTextType::new(pk.encrypt_radix(202u64, 8), pk.clone(), sk.clone());

        let res = relu(ct_1);
        assert_eq!(rck.decrypt::<u64, tfhe::shortint::ciphertext::KeyswitchBootstrap>(&res), 32768u64);
    }
}