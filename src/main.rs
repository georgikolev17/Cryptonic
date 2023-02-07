#![allow(non_snake_case)]

// Playground for encryption testing
use concrete_integer::gen_keys_radix;
use concrete_shortint::Parameters;
use concrete_shortint::parameters::{CarryModulus, MessageModulus};
use concrete_core::prelude::{DecompositionBaseLog, DecompositionLevelCount, GlweDimension, LweDimension, PolynomialSize, StandardDev};

fn main() {
    pub const _PARAM_MESSAGE_3_CARRY_1: Parameters = Parameters {
        lwe_dimension: LweDimension(259),
        glwe_dimension: GlweDimension(2),
        polynomial_size: PolynomialSize(256),
        lwe_modular_std_dev: StandardDev(0.0000043131554647504185),
        glwe_modular_std_dev: StandardDev(0.00000000000000029403601535432533),
        pbs_base_log: DecompositionBaseLog(23),
        pbs_level: DecompositionLevelCount(1),
        ks_level: DecompositionLevelCount(3),
        ks_base_log: DecompositionBaseLog(4),
        pfks_level: DecompositionLevelCount(1),
        pfks_base_log: DecompositionBaseLog(23),
        pfks_modular_std_dev: StandardDev(0.00000000000000029403601535432533),
        cbs_level: DecompositionLevelCount(0),
        cbs_base_log: DecompositionBaseLog(0),
        message_modulus: MessageModulus(8),
        carry_modulus: CarryModulus(2),
    };

    // We generate a set of client/server keys, using the default parameters:
    let num_block = 8;
    let (client_key, server_key) = gen_keys_radix(&_PARAM_MESSAGE_3_CARRY_1, num_block);

    let msg1 = 2048;
    let msg2 = 2048;

    // message_modulus^vec_length
    let modulus = client_key.parameters().message_modulus.0.pow(num_block as u32) as u64;
    println!("{modulus}");
    // We use the client key to encrypt two messages:
    let ct_1 = client_key.encrypt(msg1);
    let ct_2 = client_key.encrypt(msg2);

    // We use the server public key to execute an integer circuit:
    let ct_3 = server_key.unchecked_add(&ct_1, &ct_2);

    // We use the client key to decrypt the output of the circuit:
    let output = client_key.decrypt(&ct_3);
    println!("{output}");
    println!("{}", (msg1 + msg2) % modulus);

    assert_eq!(output, (msg1 + msg2) % modulus);
}