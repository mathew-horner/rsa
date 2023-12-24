use std::ops::Range;

use rand::Rng;
use rug::integer::{IsPrime, Order};
use rug::Integer;

use crate::math::lcm;
use crate::{PrivateKey, PublicKey};

#[derive(Debug)]
pub struct KeyPair {
    pub private: PrivateKey,
    pub public: PublicKey,
}

impl KeyPair {
    pub fn generate(opts: GenerateOpts) -> Self {
        let p = random_prime(opts.prime_size.clone());
        let q = random_prime(opts.prime_size);
        let n = p.clone() * q.clone();
        let lcm = lcm(p.clone() - Integer::from(1), q.clone() - Integer::from(1));
        let d = opts
            .exponent
            .clone()
            .pow_mod(Integer::NEG_ONE, &lcm.clone())
            .unwrap();
        Self {
            private: PrivateKey::new(p, q, d),
            public: PublicKey::new(n, opts.exponent),
        }
    }
}

pub struct GenerateOpts {
    pub exponent: Integer,
    pub prime_size: Range<u32>,
}

impl Default for GenerateOpts {
    fn default() -> Self {
        Self {
            exponent: Integer::from(65537),
            prime_size: 150..151,
        }
    }
}

fn random_prime(size: Range<u32>) -> Integer {
    let mut rng = rand::thread_rng();
    let size = rng.gen_range(size);
    loop {
        let digits: Vec<_> = (0..size).map(|_| rng.gen_range::<u8, _>(0..10)).collect();
        let integer = Integer::from_digits(&digits, Order::Lsf);
        if [IsPrime::Yes, IsPrime::Probably].contains(&integer.is_probably_prime(1)) {
            return integer;
        }
    }
}
