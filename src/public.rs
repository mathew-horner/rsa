use rug::Integer;

use crate::{decode, encode};

#[derive(Debug)]
pub struct PublicKey {
    n: Integer,
    e: Integer,
}

impl PublicKey {
    pub fn new(n: Integer, e: Integer) -> Self {
        Self { n, e }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let m = encode(data);
        let c = m.pow_mod(&self.e, &self.n).unwrap();
        decode(c)
    }
}
