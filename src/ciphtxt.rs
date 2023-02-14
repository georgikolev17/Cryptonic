 /*
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};

/// This struct is here to allow fast and easy flexibility and to limit the future problems
/// of using concrete_integer::RadixCiphertext directly.
///
pub struct CipherText{
    pub CipherTxt: Option<RadixCiphertext>,
    pub ServerKey: Option<ServerKey>,
    pub Modulus: u64,
    // This is important because when an activation such as ReLu is used we need to know
    // the encrypted value of zero
    pub CiphZero: Option<RadixCiphertext>
}

/// This impl block implements a basic constructor and a is_def() method. The is_def() method is
/// here since operations on default values return None, because there isn't a default value for
/// RadixCiphertext or for ServerKey.
///
impl CipherText{
    fn new(_CipherTxt: RadixCiphertext, _ServerKey: ServerKey, _CiphZero: RadixCiphertext) -> CipherText{
        let _l_temp = _CipherTxt.clone().blocks().get(0).unwrap().message_modulus.0 as u64;
        CipherText{
            CipherTxt: Some(_CipherTxt),
            ServerKey: Some(_ServerKey),
            Modulus: _l_temp,
            CiphZero: Some(_CiphZero)
        }
    }
    fn is_def(&self) -> bool {
        if self.CipherTxt.is_none() || self.ServerKey.is_none() || self.Modulus == 0 || self.CiphZero.is_none(){
            return true;
        }
        false
    }
}

impl Clone for CipherText{
    fn clone(&self) -> Self {
        if self.CipherTxt.is_some() && self.ServerKey.is_some() {
            CipherText::new(self.CipherTxt.clone().unwrap(), self.ServerKey.clone().unwrap(), self.CiphZero.clone().unwrap())
        }
        else{
            // Error
            CipherText::default()
        }
    }
}

/// Default trait implementation. CipherTxt and ServerKey are set to None which is then checked
/// by the ops traits and return None if any of the two are None.
impl Default for CipherText{
    fn default() -> Self {
        CipherText{
            CipherTxt: None,
            ServerKey: None,
            Modulus: 0,
            CiphZero: None
        }
    }
}

/// Implements the Add trait
impl Add for CipherText {
    type Output = Option<CipherText>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.is_def() || rhs.is_def() {
            let _ciphertext = self.ServerKey.clone().unwrap().unchecked_add(&self.CipherTxt.unwrap(), &rhs.CipherTxt.unwrap());
            Some(CipherText::new(_ciphertext, self.ServerKey.clone().unwrap(), self.CiphZero.clone().unwrap()));
        }
        None
    }
}

/// Implements the Mul trait
impl Mul for CipherText {
    type Output = Option<CipherText>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_def() || rhs.is_def() {
            let _ciphertext = self.ServerKey.clone().unwrap().smart_mul(&mut self.CipherTxt.unwrap(), &mut rhs.CipherTxt.unwrap());

            Some(CipherText::new(_ciphertext, self.ServerKey.clone().unwrap(), self.CiphZero.clone().unwrap()));

        }
        None
    }
}

/// Implements the Sub trait
impl Sub for CipherText {
    type Output = Option<CipherText>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_def() || rhs.is_def() {
            let _ciphertext = self.ServerKey.clone().unwrap().unchecked_sub(&self.CipherTxt.unwrap(), &rhs.CipherTxt.unwrap());

            Some(CipherText::new(_ciphertext, self.ServerKey.clone().unwrap(), self.CiphZero.clone().unwrap()));
        }
        None
    }
}


/// Implements the AddAssign trait
impl AddAssign for CipherText {
    fn add_assign(&mut self, rhs: Self) {
        if self.is_def() || rhs.is_def() {
            let _ciphertext = self.ServerKey.clone().unwrap().unchecked_add(&self.CipherTxt.clone().unwrap(), &rhs.CipherTxt.clone().unwrap());
            self.CipherTxt = Some(_ciphertext);
        }
    }
}

/// Implements the MulAssign trait
impl MulAssign for CipherText {
    fn mul_assign(&mut self, rhs: Self) {
        if self.is_def() || rhs.is_def() {
            let _ciphertext = self.ServerKey.clone().unwrap().smart_mul(&mut self.CipherTxt.clone().unwrap(), &mut rhs.CipherTxt.clone().unwrap());
            self.CipherTxt = Some(_ciphertext);
        }
    }
}
 */