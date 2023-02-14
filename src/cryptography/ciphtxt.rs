use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};
use tfhe::shortint::prelude::*;


/// This struct is here to allow fast and easy flexibility and to limit the future problems
/// of using concrete_integer::RadixCiphertext directly.
///
pub struct CipherTextType{
    pub CipherTxt: Option<Ciphertext>,
    pub ServerKey: Option<ServerKey>,
    pub Modulus: Option<u64>
}

/// This impl block implements a basic constructor and a is_def() method. The is_def() method is
/// here since operations on default values return None, because there isn't a default value for
/// Ciphertext or for ServerKey.
///
impl CipherTextType{
    fn new(_CipherTxt: Ciphertext, _ServerKey: ServerKey, _Modulus: u64) -> CipherTextType{
        CipherTextType{
            CipherTxt: Some(_CipherTxt),
            ServerKey: Some(_ServerKey),
            Modulus: Some(_Modulus)
        }
    }
    fn is_def(&self) -> bool {
        if self.CipherTxt.is_none() || self.ServerKey.is_none() || self.Modulus.is_none() {
            return true;
        }
        false
    }
}

impl Clone for CipherTextType{
    fn clone(&self) -> Self {
        if self.is_def() {
            CipherTextType::new(self.CipherTxt.clone().unwrap(), self.ServerKey.clone().unwrap(), self.Modulus.clone().unwrap())
        }
        else{
            // Error
            CipherTextType::default()
        }
    }
}

/// Default trait implementation. CipherTxt and ServerKey are set to None which is then checked
/// by the ops traits and return None if any of the two are None.
impl Default for CipherTextType{
    fn default() -> Self {
        CipherTextType{
            CipherTxt: None,
            ServerKey: None,
            Modulus: None
        }
    }
}

/// Implements the Add trait
impl Add for CipherTextType {
    type Output = Option<CipherTextType>;

    fn add(self, rhs: Self) -> Self::Output {
        if !self.is_def() && !rhs.is_def() {
            let _ciphertext = self.ServerKey.clone().unwrap().unchecked_add(&self.CipherTxt.unwrap(), &rhs.CipherTxt.unwrap());
            return Some(CipherTextType::new(_ciphertext, self.ServerKey.clone().unwrap(), self.Modulus.clone().unwrap()));
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

    fn mul(self, rhs: Self) -> Self::Output {
        if !self.is_def() || !rhs.is_def() {
            let _ciphertext = self.ServerKey.clone().unwrap().smart_mul_lsb(&mut self.CipherTxt.unwrap(), &mut rhs.CipherTxt.unwrap());

            Some(CipherTextType::new(_ciphertext, self.ServerKey.clone().unwrap(), self.Modulus.clone().unwrap()));

        }
        None
    }
}

/// Implements the Sub trait
impl Sub for CipherTextType {
    type Output = Option<CipherTextType>;

    fn sub(self, rhs: Self) -> Self::Output {
        if !self.is_def() || !rhs.is_def() {
            let _ciphertext = self.ServerKey.clone().unwrap().unchecked_sub(&self.CipherTxt.unwrap(), &rhs.CipherTxt.unwrap());

            Some(CipherTextType::new(_ciphertext, self.ServerKey.clone().unwrap(), self.Modulus.clone().unwrap()));
        }
        None
    }
}


/// Implements the AddAssign trait
impl AddAssign for CipherTextType {
    fn add_assign(&mut self, rhs: Self) {
        if !self.is_def() || !rhs.is_def() {
            let _ciphertext = self.ServerKey.clone().unwrap().unchecked_add(&self.CipherTxt.clone().unwrap(), &rhs.CipherTxt.clone().unwrap());
            self.CipherTxt = Some(_ciphertext);
        }
    }
}

/// Implements the MulAssign trait
impl MulAssign for CipherTextType {
    fn mul_assign(&mut self, rhs: Self) {
        if !self.is_def() || !rhs.is_def() {
            let _ciphertext = self.ServerKey.clone().unwrap().smart_mul_lsb(&mut self.CipherTxt.clone().unwrap(), &mut rhs.CipherTxt.clone().unwrap());
            self.CipherTxt = Some(_ciphertext);
        }
    }
}
