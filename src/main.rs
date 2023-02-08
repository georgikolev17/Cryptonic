#![allow(non_snake_case)]

use concrete_integer::gen_keys_radix;
use concrete_shortint::parameters::PARAM_MESSAGE_2_CARRY_2;
// Playground for encryption testing
use Cryptonic::key_gen::{_PARAM_MESSAGE_3_CARRY_1, custom_gen_keys_fixed, get_modulus};

fn main() {
/*
    // We generate a set of client/server keys, using the default parameters:
    let num_block = 8;
    let (client_key, server_key) = custom_gen_keys_fixed(num_block);

    let msg1 = 2048;
    let msg2 = 2048;

    // message_modulus^vec_length
    let modulus = get_modulus(&_PARAM_MESSAGE_3_CARRY_1, num_block);
    //println!("{modulus}");
    // We use the client key to encrypt two messages:
    let mut ct_1 = client_key.encrypt(msg1);
    let ct_2 = client_key.encrypt(msg2);

    // We use the server public key to execute an integer circuit:
    let ct_3 = server_key.smart_scalar_mul(&mut ct_1, 5);
    // We use the client key to decrypt the output of the circuit:
    let output = client_key.decrypt(&ct_3);
    println!("{output}");
    //println!("{}", (msg1 + msg2) % modulus);

    //assert_eq!(output, (msg1 + msg2) % modulus);

 */
    let modulus = 1 << 8;
    let size = 4;
    let (cks, sks) = gen_keys_radix(&PARAM_MESSAGE_2_CARRY_2, size);

    let msg = 230;
    let scalar = 376;

    let mut ct = cks.encrypt(msg);

// Compute homomorphically a scalar multiplication:
    let ct_res = sks.smart_scalar_mul(&mut ct, scalar);

// Decrypt:
    let clear = cks.decrypt(&ct_res);
    assert_eq!(msg * scalar % modulus, clear);
}
