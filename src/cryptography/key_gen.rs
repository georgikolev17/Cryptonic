// In this file I'll write the custom params for the mock examples in addition to which
// I'll also implement ser/des functions whose results will be written directly to storage
// and committed to allow faster build and test times.

use tfhe::shortint::prelude::*;


/// This is used for testing and development. Security isn't guaranteed.
pub const MY_PARAM: Parameters = Parameters {
    lwe_dimension: LweDimension(199),
    glwe_dimension: GlweDimension(1),
    polynomial_size: PolynomialSize(256),
    lwe_modular_std_dev: StandardDev(0.0000000460803851108693),
    glwe_modular_std_dev: StandardDev(0.0000000000000000002168404344971009),
    pbs_base_log: DecompositionBaseLog(15),
    pbs_level: DecompositionLevelCount(2),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(4),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(15),
    pfks_modular_std_dev: StandardDev(0.0000000000000000002168404344971009),
    cbs_level: DecompositionLevelCount(0),
    cbs_base_log: DecompositionBaseLog(0),
    message_modulus: MessageModulus(32768),
    carry_modulus: CarryModulus(16),
};

/// This function takes a short_int::Parameter and num_blocks to return the keys relating to the
/// type specified.
/// ```
/// use tfhe::shortint::prelude::*;
/// use Cryptonic::cryptography::key_gen::{custom_gen_keys, get_modulus, MY_PARAM};
///
/// let msg1 = 128;
/// let msg2 = 128;
///
/// let modulus = get_modulus(&MY_PARAM);
/// let (client_key, server_key) = custom_gen_keys(&MY_PARAM);
///
///     println!("{modulus}");
///    // We use the client key to encrypt two messages:
///    let ct_1 = client_key.encrypt(msg1);
///    let ct_2 = client_key.encrypt(msg2);
///
///    // We use the server public key to execute an integer circuit:
///    let ct_3 = server_key.unchecked_add(&ct_1, &ct_2);
///
///    // We use the client key to decrypt the output of the circuit:
///    let output = client_key.decrypt(&ct_3);
///    println!("{output}");
///    println!("{}", (msg1 + msg2) % modulus);
///
///    assert_eq!(output, (msg1 + msg2) % modulus);
/// ```
///

pub fn custom_gen_keys(parameters_set: &Parameters) -> (ClientKey, ServerKey) {
    gen_keys(MY_PARAM)
}

/// This function returns the modulus set by the parameter set and number of blocks.
///
/// Example:
///
/// ```
/// use tfhe::shortint::prelude::*;
/// use Cryptonic::cryptography::key_gen::{custom_gen_keys, get_modulus, MY_PARAM};
///
/// println!("{}", get_modulus(&MY_PARAM)); // Prints 32768
/// ```
pub fn get_modulus(parameters_set: &Parameters) -> u64 {

    // message_modulus^vec_length
    parameters_set.message_modulus.0 as u64
}