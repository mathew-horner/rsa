use std::iter::Sum;
use std::ops::{Add, Mul, Range, Sub};

use rand::Rng;

#[derive(Debug, Eq, PartialEq)]
pub struct BoundlessUint {
    digits: Vec<Digit>,
}

impl BoundlessUint {
    pub fn random_prime(digits: Range<u32>) -> Self {
        let mut rng = rand::thread_rng();
        let digits = rng.gen_range(digits);
        Self {
            digits: (0..digits)
                .map(|_| rng.gen_range(0..10))
                .map(Digit)
                .collect(),
        }
    }

    fn prepend(self, count: usize, value: Digit) -> Self {
        let mut digits: Vec<_> = (0..count).map(|_| value).collect();
        digits.extend(self.digits);
        Self { digits }
    }
}

impl<T> From<T> for BoundlessUint
where
    T: Into<u64>,
{
    fn from(number: T) -> Self {
        let mut temp: u64 = number.into();
        let mut digits = Vec::new();
        while temp > 0 {
            let digit = (temp % 10) as u8;
            temp /= 10;
            digits.push(Digit(digit));
        }
        Self { digits }
    }
}

impl Add for &BoundlessUint {
    type Output = BoundlessUint;

    fn add(self, rhs: Self) -> Self::Output {
        let zero = Digit(0);
        let mut lhs: Vec<_> = self.digits.iter().collect();
        let mut rhs: Vec<_> = rhs.digits.iter().collect();
        let size = lhs.len().max(rhs.len());
        lhs.resize(size, &zero);
        rhs.resize(size, &zero);
        let mut digits = Vec::new();
        let mut carry = 0;

        for (left, right) in lhs.into_iter().zip(rhs) {
            let mut next = carry + left.0 + right.0;
            carry = next / 10;
            next %= 10;
            digits.push(Digit(next));
        }

        if carry > 0 {
            digits.push(Digit(carry))
        }

        BoundlessUint { digits }
    }
}

impl Sum for BoundlessUint {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(BoundlessUint::from(0_u64), |acc, n| &acc + &n)
    }
}

impl Sub for &BoundlessUint {
    type Output = BoundlessUint;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut lhs: Vec<_> = self.digits.clone().into_iter().collect();
        let mut rhs: Vec<_> = rhs.digits.clone().into_iter().collect();
        let size = lhs.len().max(rhs.len());
        lhs.resize(size, Digit(0));
        rhs.resize(size, Digit(0));
        let mut digits = Vec::new();

        for idx in 0..size {
            let mut left = lhs[idx].0;
            let right = rhs[idx].0;

            if left < right {
                // NOTE: This will panic if lhs < rhs, but we assume the difference is going to be positive.
                let mut j = 1;
                while lhs[idx + j] == 0 {
                    lhs[idx + j] = Digit(9);
                    j += 1;
                }
                lhs[idx + j].0 -= 1;
                left += 10;
            }

            digits.push(Digit(left - right));
        }

        BoundlessUint { digits }
    }
}

impl Mul for &BoundlessUint {
    type Output = BoundlessUint;

    fn mul(self, rhs: Self) -> Self::Output {
        rhs.digits
            .iter()
            .enumerate()
            .map(|(iter, digit)| (self * digit).prepend(iter, Digit(0)))
            .sum()
    }
}

impl Mul<&Digit> for &BoundlessUint {
    type Output = BoundlessUint;

    fn mul(self, rhs: &Digit) -> Self::Output {
        let mut digits = Vec::new();
        let mut carry = 0;

        for digit in self.digits.iter() {
            let mut next = carry + digit.0 * rhs.0;
            carry = next / 10;
            next %= 10;
            digits.push(Digit(next));
        }

        if carry > 0 {
            digits.push(Digit(carry))
        }

        BoundlessUint { digits }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Digit(u8);

impl PartialEq<u8> for Digit {
    fn eq(&self, other: &u8) -> bool {
        &self.0 == other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_number() {
        assert_eq!(
            BoundlessUint::from(120_u64),
            BoundlessUint {
                digits: vec![Digit(0), Digit(2), Digit(1)]
            }
        );
    }

    #[test]
    fn add_boundless() {
        assert_eq!(
            &BoundlessUint::from(821_u64) + &BoundlessUint::from(278_u64),
            BoundlessUint::from(1099_u64)
        );
    }

    #[test]
    fn sub_boundless() {
        assert_eq!(
            &BoundlessUint::from(6500_u64) - &BoundlessUint::from(469_u64),
            BoundlessUint::from(6031_u64)
        );
    }

    #[test]
    fn mul_boundless() {
        assert_eq!(
            &BoundlessUint::from(3421_u64) * &BoundlessUint::from(875_u64),
            BoundlessUint::from(2993375_u64),
        )
    }

    #[test]
    fn multiply_boundless_and_digit() {
        assert_eq!(
            &BoundlessUint::from(256_u64) * &Digit(5),
            BoundlessUint::from(1280_u64)
        );
    }
}
