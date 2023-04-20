extern crate core;

#[cfg(test)]
mod test_neural_network {
    use std::ptr::null;
    use Cryptonic::neural_network::nnet::Nnet;

    #[test]
    fn test_adding_a_new_layer() {
        let nn : Nnet<i32> = Nnet::new();

    }
}


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

        assert!(ct_3.is_some());
        assert_eq!(rck.decrypt::<u64, tfhe::shortint::ciphertext::KeyswitchBootstrap>(&ct_3.unwrap().CipherTxt), 25u64);
    }

    #[test]
    fn test_ciphertext_mul(){
        let (rck, sk, pk) = custom_gen_keys();
        let ct_1 = CipherTextType::new(pk.encrypt_radix(202u64, 8), pk.clone(), sk.clone());
        let ct_2 = CipherTextType::new(pk.encrypt_radix(10u64, 8), pk, sk);

        let ct_3 = ct_1 * ct_2;

        assert!(ct_3.is_some());
        assert_eq!(rck.decrypt::<u64, tfhe::shortint::ciphertext::KeyswitchBootstrap>(&ct_3.unwrap().CipherTxt), 2020u64);
    }

    #[test]
    fn test_ciphertext_add_i32(){
        let (rck, sk, pk) = custom_gen_keys();
        let ct_1 = CipherTextType::new(pk.encrypt_radix(202u64, 8), pk.clone(), sk.clone());

        let ct_3 = ct_1 + 50i32;

        assert!(ct_3.is_some());
        assert_eq!(rck.decrypt::<u64, tfhe::shortint::ciphertext::KeyswitchBootstrap>(&ct_3.unwrap().CipherTxt), 252u64);
    }

    #[test]
    fn test_ciphertext_mul_i32(){
        let (rck, sk, pk) = custom_gen_keys();
        let ct_1 = CipherTextType::new(pk.encrypt_radix(202u64, 8), pk.clone(), sk.clone());

        let ct_3 = ct_1 * 50i32;

        assert!(ct_3.is_some());
        assert_eq!(rck.decrypt::<u64, tfhe::shortint::ciphertext::KeyswitchBootstrap>(&ct_3.unwrap().CipherTxt), 10100u64);
    }
}

#[cfg(test)]
mod test_layers {
    use Cryptonic::neural_network::dense_layer::DenseLayer;
    use Cryptonic::neural_network::layer_trait::Layer;
    use Cryptonic::neural_network::nnet::Nnet;

    #[test]
    fn test_layer_shape() {
        let layer : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);
        let input_shape : Vec<usize> = vec![3, 2];
        assert_eq!(layer.get_input_shape(), &input_shape);
    }

}

/*
#[cfg(test)]
mod test_layers
{
    use crate::dense_layer;
    #[test]
    fn test_dense_layer_weights_and_biases() {
        let dense_layer = dense_layer::DenseLayer::new(20, 10);
        assert_eq!(10, dense_layer.weights.len());
        assert_eq!(20, dense_layer.weights[0].len());
        assert_eq!(10, dense_layer.biases.len());
    }
}
*/
