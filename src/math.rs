use rug::Integer;

pub fn gcd(mut n1: Integer, mut n2: Integer) -> Integer {
    while n1 != n2 {
        if n1 > n2 {
            n1 = n1.clone() - n2.clone();
        } else {
            n2 = n2.clone() - n1.clone();
        }
    }
    n1
}

pub fn lcm(n1: Integer, n2: Integer) -> Integer {
    (n1.clone() * n2.clone()) / gcd(n1, n2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_works() {
        assert_eq!(gcd(Integer::from(24), Integer::from(18)), Integer::from(6));
    }

    #[test]
    fn lcm_works() {
        assert_eq!(lcm(Integer::from(24), Integer::from(18)), Integer::from(72));
    }
}
