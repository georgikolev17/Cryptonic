use crate::cryptography::ciphtxt::CipherTextType;
use tfhe::integer::ciphertext::RadixCiphertext;
use tfhe::integer::RadixClientKey;
use tfhe::shortint::ciphertext::KeyswitchBootstrap;
use crate::cryptography::key_gen::custom_gen_keys;

// Considering we're working with 16-bit unsigned integers our range
// will be from 0-65536, with a midpoint at 32,768 which we'll consider
// as our zero
pub fn binary_step(mut item: CipherTextType) -> RadixCiphertext<KeyswitchBootstrap>{

    let (rck, sk, pk) = custom_gen_keys();

    let mut zero = item.PublicKey.encrypt_radix(32768u64, 8);
    let mut one = item.PublicKey.encrypt_radix(65536u64, 8);
    let res = item.ServerKey.smart_lt(&mut item.CipherTxt, &mut zero);
    let bool_res = rck.decrypt::<u64, KeyswitchBootstrap>(&res);
    if bool_res == 1u64 {
        return zero;
    }
    one
}


pub fn relu(mut elem:CipherTextType) -> RadixCiphertext<KeyswitchBootstrap>{
    let (rck, sk, pk) = custom_gen_keys();

    let mut zero = elem.PublicKey.encrypt_radix(32768u64, 8);
    let res = elem.ServerKey.smart_lt(&mut elem.CipherTxt, &mut zero);
    let bool_res = rck.decrypt::<u64, KeyswitchBootstrap>(&res);
    if bool_res == 1u64 {
        return elem.PublicKey.encrypt_radix(32768u64, 8);
    }
    return elem.PublicKey.encrypt_radix(65536u64, 8);
}