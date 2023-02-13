// // In this file I'll write the custom params for the mock examples in addition to which
// // I'll also implement ser/des functions whose results will be written directly to storage
// // and committed to allow faster build and test times.
//
// use concrete_integer::{gen_keys_radix, RadixClientKey, ServerKey};
// use concrete_shortint::Parameters;
// use concrete_shortint::parameters::{CarryModulus, MessageModulus};
// use concrete_core::prelude::{DecompositionBaseLog, DecompositionLevelCount, GlweDimension, LweDimension, PolynomialSize, StandardDev};
//
// pub const _PARAM_MESSAGE_3_CARRY_1: Parameters = Parameters {
//     // We changed this parameter
//     lwe_dimension: LweDimension(259),
//     glwe_dimension: GlweDimension(2),
//     // We changed this parameter
//     polynomial_size: PolynomialSize(256),
//     lwe_modular_std_dev: StandardDev(0.0000043131554647504185),
//     glwe_modular_std_dev: StandardDev(0.00000000000000029403601535432533),
//     pbs_base_log: DecompositionBaseLog(23),
//     pbs_level: DecompositionLevelCount(1),
//     ks_level: DecompositionLevelCount(3),
//     ks_base_log: DecompositionBaseLog(4),
//     pfks_level: DecompositionLevelCount(1),
//     pfks_base_log: DecompositionBaseLog(23),
//     pfks_modular_std_dev: StandardDev(0.00000000000000029403601535432533),
//     cbs_level: DecompositionLevelCount(0),
//     cbs_base_log: DecompositionBaseLog(0),
//     message_modulus: MessageModulus(8),
//     carry_modulus: CarryModulus(2),
// };
//
// /// This function takes a short_int::Parameter and num_blocks to return the keys relating to the
// /// type specified.
// /// ```
// /// use concrete_integer::{gen_keys_radix, RadixClientKey, ServerKey};
// /// use concrete_shortint::Parameters;
// /// use concrete_shortint::parameters::{CarryModulus, MessageModulus};
// /// use concrete_core::prelude::{DecompositionBaseLog, DecompositionLevelCount, GlweDimension, LweDimension, PolynomialSize, StandardDev};
// /// use Cryptonic::key_gen::{custom_gen_keys, get_modulus, _PARAM_MESSAGE_3_CARRY_1};
// ///
// /// let msg1 = 128;
// /// let msg2 = 128;
// ///
// /// let modulus = get_modulus(&_PARAM_MESSAGE_3_CARRY_1, 4);
// /// let (client_key, server_key) = custom_gen_keys(&_PARAM_MESSAGE_3_CARRY_1, 4);
// ///
// ///     println!("{modulus}");
// ///    // We use the client key to encrypt two messages:
// ///    let ct_1 = client_key.encrypt(msg1);
// ///    let ct_2 = client_key.encrypt(msg2);
// ///
// ///    // We use the server public key to execute an integer circuit:
// ///    let ct_3 = server_key.unchecked_add(&ct_1, &ct_2);
// ///
// ///    // We use the client key to decrypt the output of the circuit:
// ///    let output = client_key.decrypt(&ct_3);
// ///    println!("{output}");
// ///    println!("{}", (msg1 + msg2) % modulus);
// ///
// ///    assert_eq!(output, (msg1 + msg2) % modulus);
// /// ```
// ///
//
// pub fn custom_gen_keys(parameters_set: &Parameters, num_blocks: usize) -> (RadixClientKey, ServerKey) {
//     gen_keys_radix(parameters_set, num_blocks)
// }
//
// /// This function fixes the parameter set to Cryptonic::key_gen::_PARAM_MESSAGE_3_CARRY_1 and only
// /// allows the changing of the num_blocks parameter.
// /// ```
// /// use concrete_integer::{gen_keys_radix, RadixClientKey, ServerKey};
// /// use concrete_shortint::Parameters;
// /// use concrete_shortint::parameters::{CarryModulus, MessageModulus};
// /// use concrete_core::prelude::{DecompositionBaseLog, DecompositionLevelCount, GlweDimension, LweDimension, PolynomialSize, StandardDev};
// /// use Cryptonic::key_gen::{custom_gen_keys, get_modulus, _PARAM_MESSAGE_3_CARRY_1};
// ///
// /// let msg1 = 128;
// /// let msg2 = 128;
// ///
// /// let modulus = get_modulus(&_PARAM_MESSAGE_3_CARRY_1, 4);
// /// let (client_key, server_key) = custom_gen_keys(&_PARAM_MESSAGE_3_CARRY_1, 4);
// ///
// ///     println!("{modulus}");
// ///    // We use the client key to encrypt two messages:
// ///    let ct_1 = client_key.encrypt(msg1);
// ///    let ct_2 = client_key.encrypt(msg2);
// ///
// ///    // We use the server public key to execute an integer circuit:
// ///    let ct_3 = server_key.unchecked_add(&ct_1, &ct_2);
// ///
// ///    // We use the client key to decrypt the output of the circuit:
// ///    let output = client_key.decrypt(&ct_3);
// ///    println!("{output}");
// ///    println!("{}", (msg1 + msg2) % modulus);
// ///
// ///    assert_eq!(output, (msg1 + msg2) % modulus);
// /// ```
// ///
//
// pub fn custom_gen_keys_fixed(num_blocks: usize) -> (RadixClientKey, ServerKey) {
//     gen_keys_radix(&_PARAM_MESSAGE_3_CARRY_1, num_blocks)
// }
//
// /// This function returns the modulus set by the parameter set and number of blocks.
// ///
// /// Example:
// ///
// /// ```
// /// use concrete_integer::{gen_keys_radix, RadixClientKey, ServerKey};
// /// use concrete_shortint::Parameters;
// /// use concrete_shortint::parameters::{CarryModulus, MessageModulus};
// /// use concrete_core::prelude::{DecompositionBaseLog, DecompositionLevelCount, GlweDimension, LweDimension, PolynomialSize, StandardDev};
// /// use Cryptonic::key_gen::{_PARAM_MESSAGE_3_CARRY_1, get_modulus};
// ///
// /// let num_block = 4;
// /// println!("{}", get_modulus(&_PARAM_MESSAGE_3_CARRY_1, 4)); // Prints 4096 = MessageModulus^num_block
// /// ```
// pub fn get_modulus(parameters_set: &Parameters, num_blocks: usize) -> u64 {
//
//     // message_modulus^vec_length
//     parameters_set.message_modulus.0.pow(num_blocks as u32) as u64
// }