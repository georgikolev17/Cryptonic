use tfhe::integer::ciphertext::{RadixCiphertext};
use tfhe::integer::{RadixClientKey, ServerKey, PublicKeyBig};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};
use tfhe::shortint::ciphertext::KeyswitchBootstrap;
use crate::cryptography::key_gen::custom_gen_keys;

/// This struct is here to allow fast and easy flexibility and to limit the future problems
/// of using concrete_integer::RadixCiphertext directly.
///
#[derive(Clone)]
pub struct CipherTextType{
    pub CipherTxt: RadixCiphertext<KeyswitchBootstrap>,
    pub PublicKey: PublicKeyBig,
    pub ServerKey: ServerKey,
    pub Zero: RadixCiphertext<KeyswitchBootstrap>,
    pub IfDefRandClientKey: Option<RadixClientKey>
}

/// This impl block implements a basic constructor and a is_def() method. The is_def() method is
/// here since operations on default values return None, because there isn't a default value for
/// Ciphertext or for ServerKey.
///
impl CipherTextType{
    pub fn new(_CipherTxt: RadixCiphertext<KeyswitchBootstrap>, _PublicKey: PublicKeyBig, _ServerKey: ServerKey) -> CipherTextType{
        CipherTextType{
            CipherTxt: _CipherTxt,
            PublicKey: _PublicKey.clone(),
            ServerKey: _ServerKey,
            Zero: _PublicKey.clone().encrypt_radix(0u64, 8),
            IfDefRandClientKey: None
        }
    }
    pub fn is_def(&self) -> bool {
        if self.IfDefRandClientKey.is_some() {
            return true;
        }
        false
    }
}


/// Default trait implementation. CipherTxt and ServerKey are set to None which is then checked
/// by the ops traits and return None if any of the two are None.
impl Default for CipherTextType{
    fn default() -> Self {
        let (ck, sk, pk) = custom_gen_keys();
        CipherTextType{
            CipherTxt: pk.clone().encrypt_radix(0u64, 8),
            PublicKey: pk.clone(),
            ServerKey: sk,
            Zero: pk.clone().encrypt_radix(0u64, 8),
            IfDefRandClientKey: Some(ck)
        }
    }
}

/// Implements the Add trait
impl Add for CipherTextType {
    type Output = Option<CipherTextType>;

    fn add(self, rhs: Self) -> Self::Output {
        if !self.is_def() && !rhs.is_def() {
            let _sum = self.ServerKey.unchecked_add(&self.CipherTxt, &rhs.CipherTxt);
            return Some(CipherTextType::new(_sum, self.PublicKey, self.ServerKey));
        }
        else if !self.is_def() && rhs.is_def() {
            return Some(self.clone());
        }
        else if self.is_def() && !rhs.is_def() {
            return Some(rhs.clone());
        }
        None
    }
}

/// Implements the Mul trait
impl Mul for CipherTextType {
    type Output = Option<CipherTextType>;

    fn mul(mut self, mut rhs: Self) -> Self::Output {

        if !self.is_def() && !rhs.is_def() {
            let _ciphertext = self.ServerKey.smart_mul(&mut self.CipherTxt, &mut rhs.CipherTxt);

            return Some(CipherTextType::new(_ciphertext, self.PublicKey, self.ServerKey));
        }
        None
    }
}

/// Implements the Sub trait
impl Sub for CipherTextType {
    type Output = Option<CipherTextType>;

    fn sub(self, rhs: Self) -> Self::Output {
        if !self.is_def() || !rhs.is_def() {
            let _ciphertext = self.ServerKey.unchecked_sub(&self.CipherTxt, &rhs.CipherTxt);

            return Some(CipherTextType::new(_ciphertext, self.PublicKey, self.ServerKey));
        }
        None
    }
}


/// Implements the AddAssign trait
impl AddAssign for CipherTextType {
    fn add_assign(&mut self, rhs: Self) {
        if !self.is_def() || !rhs.is_def() {
            let _ciphertext = self.ServerKey.unchecked_add(&self.CipherTxt.clone(), &rhs.CipherTxt.clone());
            self.CipherTxt = _ciphertext;
        }
    }
}

/// Implements the MulAssign trait
impl MulAssign for CipherTextType {
    fn mul_assign(&mut self, rhs: Self) {
        if !self.is_def() || !rhs.is_def() {
            let _ciphertext = self.ServerKey.smart_mul(&mut self.CipherTxt.clone(), &mut rhs.CipherTxt.clone());
            self.CipherTxt = _ciphertext;
        }
    }
}




/// Implements the Add trait
impl Add<i32> for CipherTextType {
    type Output = Option<CipherTextType>;

    fn add(self, rhs: i32) -> Self::Output {
        if !self.is_def() {
            let _sum = self.ServerKey.unchecked_scalar_add(&self.CipherTxt, rhs as u64);
            return Some(CipherTextType::new(_sum, self.PublicKey, self.ServerKey));
        }
        None
    }
}

/// Implements the Mul trait
impl Mul<i32> for CipherTextType {
    type Output = Option<CipherTextType>;

    fn mul(mut self, rhs: i32) -> Self::Output {
        if !self.is_def() {
            let _ciphertext = self.ServerKey.smart_mul(&mut self.CipherTxt, &mut self.PublicKey.encrypt_radix(rhs as u64, 8));

            return Some(CipherTextType::new(_ciphertext, self.PublicKey, self.ServerKey));
        }
        None
    }
}

/// Implements the Sub trait
impl Sub<i32> for CipherTextType {
    type Output = Option<CipherTextType>;

    fn sub(self, rhs: i32) -> Self::Output {
        if !self.is_def() {
            let _ciphertext = self.ServerKey.unchecked_scalar_sub(&self.CipherTxt, rhs as u64);

            return Some(CipherTextType::new(_ciphertext, self.PublicKey, self.ServerKey));
        }
        None
    }
}


/// Implements the AddAssign trait
impl AddAssign<i32> for CipherTextType {
    fn add_assign(&mut self, rhs: i32) {
        if !self.is_def() {
            let _ciphertext = self.ServerKey.unchecked_scalar_add(&self.CipherTxt.clone(), rhs as u64);
            self.CipherTxt = _ciphertext;
        }
    }
}

/// Implements the MulAssign trait
impl MulAssign<i32> for CipherTextType {
    fn mul_assign(&mut self, rhs: i32) {
        if !self.is_def() {
            let _ciphertext = self.ServerKey.smart_mul(&mut self.CipherTxt.clone(), &mut self.PublicKey.encrypt_radix(rhs as u64, 8));
            self.CipherTxt = _ciphertext;
        }
    }
}
