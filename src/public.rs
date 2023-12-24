use crate::error::EncryptError;
use crate::BoundlessUint;

pub struct PublicKey {
    /// The modulus (p * q from the private key).
    n: BoundlessUint,
    /// The public key exponent.
    e: BoundlessUint,
}

impl PublicKey {
    pub fn new(n: BoundlessUint, e: BoundlessUint) -> Self {
        Self { n, e }
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, EncryptError> {
        todo!();
    }
}
