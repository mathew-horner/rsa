use rug::Integer;

use crate::{decode, encode};

#[derive(Debug)]
pub struct PrivateKey {
    p: Integer,
    q: Integer,
    d: Integer,
}

impl PrivateKey {
    pub fn new(p: Integer, q: Integer, d: Integer) -> Self {
        Self { p, q, d }
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        let c = encode(data);
        let n = self.p.clone() * self.q.clone();
        let m = c.pow_mod(&self.d, &n).unwrap();
        decode(m)
    }
}
