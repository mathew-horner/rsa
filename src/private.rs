use crate::error::DecryptError;
use crate::BoundlessUint;

pub struct PrivateKey {
    /// The first large prime that this key is made of.
    p: BoundlessUint,
    /// The second large prime that this key is made of.
    q: BoundlessUint,
    /// The value of Carmichael's Totient Function over `n`, which is equivalent to `lcm(p - 1, q - 1)`.
    lcm: BoundlessUint,
    /// The private key exponent.
    d: BoundlessUint,
}

impl PrivateKey {
    pub fn new(p: BoundlessUint, q: BoundlessUint, lcm: BoundlessUint, d: BoundlessUint) -> Self {
        Self { p, q, lcm, d }
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, DecryptError> {
        todo!();
    }
}
