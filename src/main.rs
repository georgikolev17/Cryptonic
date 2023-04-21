// use ndarray::{Array, array, Ix1, IxDyn};
// use Cryptonic::cryptography::ciphtxt;
// use Cryptonic::cryptography::ciphtxt::CipherTextType;
// use Cryptonic::cryptography::key_gen::{custom_gen_keys, first_key_gen_once};

use Cryptonic::examples::example_net::and_gate_net;

fn main() {
    // // Define the linear array
    // let arr_linear = Array::from(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    //
    // // Define the dynamic dimension array
    // let arr_dyn = Array::from_shape_vec(IxDyn(&[3, 2]), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
    // let a=2;
    // // Convert the dynamic dimension array to a fixed dimension array
    // let shape = arr_dyn.shape();
    // let res = shape.iter().fold(1, |acc, x| acc * x);
    //
    // println!("{}", res);
    // let arr_fixed = arr_dyn.into_shape((6)).unwrap();
    //
    // // Find the dot product
    // let res = arr_fixed.dot(&arr_linear);
    //
    // println!("{:?}", res);

    // let (rck, sk, pk) = custom_gen_keys();
    // let raw_cipher_text1 = pk.encrypt_radix(5u64, 8);
    // let raw_cipher_text2 = pk.encrypt_radix(10u64, 8);
    // let raw_cipher_text3 = pk.encrypt_radix(15u64, 8);
    // let raw_cipher_text4 = pk.encrypt_radix(20u64, 8);
    //
    // let myCipherTxt1 = CipherTextType::new(raw_cipher_text1, pk.clone(), sk.clone());
    // let myCipherTxt2 = CipherTextType::new(raw_cipher_text2, pk.clone(), sk.clone());
    // let myCipherTxt3 = CipherTextType::new(raw_cipher_text3, pk.clone(), sk.clone());
    // let myCipherTxt4 = CipherTextType::new(raw_cipher_text4, pk.clone(), sk.clone());
    //
    //
    // let arr_cr1 = array![myCipherTxt1, myCipherTxt2];
    // let arr_cr2 = array![myCipherTxt3, myCipherTxt4];
    //
    // let arr_i32_1 = array![1i32,5i32];
    //
    // let res = arr_cr1.clone() + arr_i32_1.clone();
    // let mul = arr_cr1 * arr_i32_1;
    // //let dot = arr_cr1.dot(&arr_cr2);
    // for elem in mul {
    //     println!("{:?}", rck.decrypt::<u64, tfhe::shortint::ciphertext::KeyswitchBootstrap>(&elem.CipherTxt));
    // }
    and_gate_net(true, true);
    and_gate_net(true, false);
    and_gate_net(false, true);
    and_gate_net(false, false);

}

