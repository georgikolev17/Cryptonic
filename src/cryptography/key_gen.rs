// In this file I'll write the custom params for the mock examples in addition to which
// I'll also implement ser/des functions whose results will be written directly to storage
// and committed to allow faster build and test times.

//use tfhe::shortint::prelude::*;
//use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2;
use tfhe::integer::{gen_keys_radix, RadixClientKey, ServerKey};
use tfhe::integer::parameters::{LweDimension,GlweDimension,PolynomialSize,StandardDev,DecompositionBaseLog,DecompositionLevelCount,CiphertextModulus};
use tfhe::integer::PublicKeyBig;
use tfhe::shortint::{CarryModulus, MessageModulus, Parameters};

/// This is used for testing and development. Security isn't guaranteed.
pub const CUSTOM_PARAM_MESSAGE_2_CARRY_2: Parameters = Parameters {
    lwe_dimension: LweDimension(53),
    glwe_dimension: GlweDimension(1),
    polynomial_size: PolynomialSize(128),
    lwe_modular_std_dev: StandardDev(0.000007069849454709433),
    glwe_modular_std_dev: StandardDev(0.00000000000000029403601535432533),
    pbs_base_log: DecompositionBaseLog(23),
    pbs_level: DecompositionLevelCount(1),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(3),
    pfks_level: DecompositionLevelCount(1),
    pfks_base_log: DecompositionBaseLog(23),
    pfks_modular_std_dev: StandardDev(0.00000000000000029403601535432533),
    cbs_level: DecompositionLevelCount(0),
    cbs_base_log: DecompositionBaseLog(0),
    message_modulus: MessageModulus(4),
    carry_modulus: CarryModulus(4),
    ciphertext_modulus: CiphertextModulus::new_native(),
};


pub fn custom_gen_keys() -> (RadixClientKey, ServerKey,PublicKeyBig) {
    let (client_key, server_key) = gen_keys_radix(&CUSTOM_PARAM_MESSAGE_2_CARRY_2, 8);
    let public_key = PublicKeyBig::new(&client_key);
    (client_key, server_key, public_key)
}
