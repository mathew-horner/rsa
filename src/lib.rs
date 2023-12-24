mod math;
mod pair;
mod private;
mod public;

use rug::Integer;

pub use pair::{GenerateOpts, KeyPair};
pub use private::PrivateKey;
pub use public::PublicKey;

pub(crate) fn encode(data: &[u8]) -> Integer {
    data.iter()
        .fold(Integer::new(), |int, byte| int * 256 + byte)
}

pub(crate) fn decode(int: Integer) -> Vec<u8> {
    let mut bytes: Vec<_> = int
        .as_limbs()
        .iter()
        .map(ToOwned::to_owned)
        .flat_map(u64::to_le_bytes)
        .collect();

    bytes.truncate(bytes.len() - bytes.iter().rev().take_while(|&&n| n == 0).count());
    bytes.reverse();
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_works() {
        let data = &[0b10101100, 0b01100110, 0b11010010, 0b00101000];
        let encoded = encode(data);
        let decoded = decode(encoded);
        assert_eq!(&decoded, data);
    }

    #[test]
    fn asymmetric_encryption_works() {
        let message = "Hello World!";
        let pair = KeyPair::generate(GenerateOpts::default());
        let encrypted = pair.public.encrypt(message.as_bytes());
        let decrypted = pair.private.decrypt(&encrypted);
        assert_eq!(String::from_utf8(decrypted).unwrap(), message);
    }
}
