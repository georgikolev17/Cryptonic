use concrete_integer::{gen_keys_radix, IntegerCiphertext, RadixCiphertext, RadixClientKey, ServerKey};
use concrete_shortint::Parameters;
use concrete_shortint::parameters::{CarryModulus, MessageModulus};
use concrete_core::prelude::{DecompositionBaseLog, DecompositionLevelCount, GlweDimension, LweDimension, PolynomialSize, StandardDev};

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};

struct CipherText{
    CipherTxt: Option<concrete_integer::RadixCiphertext>,
    ServerKey: Option<concrete_integer::ServerKey>,
    Modulus: u64
}

impl CipherText{
    fn new(_CipherTxt: RadixCiphertext, _ServerKey: ServerKey) -> CipherText{
        let _l_temp = _CipherTxt.clone().blocks().get(0).unwrap().message_modulus.0 as u64;
        CipherText{
            CipherTxt: Some(_CipherTxt),
            ServerKey: Some(_ServerKey),
            Modulus: _l_temp
        }
    }
    fn is_def(&self) -> bool {
        if self.CipherTxt.is_none() || self.ServerKey.is_none() || self.Modulus == 0{
            return true;
        }
        false
    }
}

impl Clone for CipherText{
    fn clone(&self) -> Self {
        if self.CipherTxt.is_some() && self.ServerKey.is_some() {
            CipherText::new(self.CipherTxt.unwrap().clone(), self.ServerKey.unwrap().clone())
        }
        else{
            // Error
            CipherText::default()
        }
    }
}

impl Default for CipherText{
    fn default() -> Self {
        CipherText{
            CipherTxt: None,
            ServerKey: None,
            Modulus: 0
        }
    }
}

impl Add for CipherText {
    type Output = Option<CipherText>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.is_def() || rhs.is_def() || self.ServerKey.unwrap() != rhs.ServerKey.unwrap() {
            Some(self.ServerKey.unwrap().unchecked_add(&self.CipherTxt.unwrap(), &rhs.CipherTxt.unwrap()))
        }
        None
    }
}

impl Mul for CipherText {
    type Output = Option<CipherText>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_def() || rhs.is_def() || self.ServerKey.unwrap() != rhs.ServerKey.unwrap() {
            Some(self.ServerKey.unwrap().smart_mul(&mut self.CipherTxt.unwrap(), &mut rhs.CipherTxt.unwrap()))
        }
        None
    }
}

impl Sub for CipherText {
    type Output = Option<CipherText>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_def() || rhs.is_def() || self.ServerKey.unwrap() != rhs.ServerKey.unwrap() {
            Some(self.ServerKey.unwrap().unchecked_sub(&self.CipherTxt.unwrap(), &rhs.CipherTxt.unwrap()))
        }
        None
    }
}