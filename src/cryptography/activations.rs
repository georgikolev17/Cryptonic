use tfhe::integer::ciphertext::RadixCiphertext;
use tfhe::shortint::ciphertext::KeyswitchBootstrap;
use crate::cryptography::ciphtxt::CipherTextType;

// Considering we're working with 16-bit unsigned integers our range
// will be from 0-65536, with a midpoint at 32,768 which we'll consider
// as our zero
pub fn binary_step(mut item: CipherTextType) -> RadixCiphertext<KeyswitchBootstrap>{

    let mut zero = item.PublicKey.encrypt_radix(32768u64, 8);
    let mut one = item.PublicKey.encrypt_radix(65536u64, 8);

    if item.ServerKey.smart_lt(&mut item, &mut zero) {
        return zero;
    }
    one
}

pub fn relu(mut item: CipherTextType) -> RadixCiphertext<KeyswitchBootstrap>{
    let mut zero = item.PublicKey.encrypt_radix(32768u64, 8);

    if item.ServerKey.smart_lt(&mut item, &mut zero) {
        return zero;
    }
    item.CipherTxt
}