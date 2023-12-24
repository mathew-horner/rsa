use std::ops::Range;

use crate::math::lcm;
use crate::{BoundlessUint, PrivateKey, PublicKey};

pub struct KeyPair {
    pub private: PrivateKey,
    pub public: PublicKey,
}

impl KeyPair {
    /// Create a new public / private key pair.
    pub fn generate(opts: GenerateOpts) -> Self {
        let p = BoundlessUint::random_prime(opts.prime_size.clone());
        let q = BoundlessUint::random_prime(opts.prime_size);
        let n = &p * &q;
        let lcm = lcm(
            &p - &BoundlessUint::from(1_u64),
            &q - &BoundlessUint::from(1_u64),
        );
        let d = BoundlessUint::from(1_u64);
        Self {
            private: PrivateKey::new(p, q, lcm, d),
            public: PublicKey::new(n, opts.exponent),
        }
    }
}

pub struct GenerateOpts {
    pub exponent: BoundlessUint,
    pub prime_size: Range<u32>,
}

impl Default for GenerateOpts {
    fn default() -> Self {
        Self {
            exponent: BoundlessUint::from(65537_u64),
            prime_size: 150..151,
        }
    }
}
