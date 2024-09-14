//! # nybble
//!
//! `nybble` is a helper crate to split byte vectors into nybbles
//! and combine them back.

use std::fmt;

/// The order of nybbles in a byte.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum NybbleOrder {
    HighFirst,
    LowFirst
}

impl fmt::Display for NybbleOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            NybbleOrder::HighFirst => "h",
            NybbleOrder::LowFirst => "l",
        };
        write!(f, "{}", s)
    }
}

/// Gets the high nybble from a byte.
pub fn high_nybble(b: u8) -> u8 {
    (b & 0xf0) >> 4
}

/// Gets the low nybble from a byte.
pub fn low_nybble(b: u8) -> u8 {
    b & 0x0f
}

/// Gets the high and low nybble of a byte as a tuple,
/// with the high nybble first.
pub fn nybbles_from_byte(b: u8) -> (u8, u8) {
    (high_nybble(b), low_nybble(b))
}

/// Makes a byte from the high and low nybbles,
/// with the high nybble specified first.
pub fn byte_from_nybbles(high: u8, low: u8) -> u8 {
    high << 4 | low
}

/// Make a new byte array from `data` with the bytes split into
/// high and low nybbles. The `order` argument determines
/// which one comes first.
pub fn nybblify(data: Vec<u8>, order: NybbleOrder) -> Vec<u8> {
    let mut result = Vec::<u8>::new();

    for b in data {
        let n = nybbles_from_byte(b);
        if order == NybbleOrder::HighFirst {
            result.push(n.0);
            result.push(n.1);
        } else {
            result.push(n.1);
            result.push(n.0);
        }
    }

    result
}

/// Make a new byte array from `data` by combining adjacent bytes
/// representing the high and low nybbles of each byte.
/// The `order` argument determines which one comes first.
pub fn denybblify(data: Vec<u8>, order: NybbleOrder) -> Vec<u8> {
    assert_eq!(data.len() % 2, 0);  // length must be even

    let mut result = Vec::<u8>::new();

    let mut index = 0;
    let mut offset = 0;
    let count = data.len() / 2;

    while index < count {
        let high = data[offset];
        let low = data[offset + 1];
        let b = if order == NybbleOrder::HighFirst {
            byte_from_nybbles(high, low)
        } else {
            byte_from_nybbles(low, high)
        };
        result.push(b);
        index += 1;
        offset += 2;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nybblify() {
        let b = vec![0x01, 0x23, 0x45];
        let nb = nybblify(b, NybbleOrder::HighFirst);
        assert_eq!(nb, vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);
    }

    #[test]
    fn test_nybblify_flipped() {
        let b = vec![0x57, 0x61, 0x76];
        let nb = nybblify(b, NybbleOrder::LowFirst);
        assert_eq!(nb, vec![0x07, 0x05, 0x01, 0x06, 0x06, 0x07]);
    }

    #[test]
    fn test_denybblify() {
        let b = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
        let nb = denybblify(b, NybbleOrder::HighFirst);
        assert_eq!(nb, vec![0x01, 0x23, 0x45]);
    }

    #[test]
    fn test_denybblify_flipped() {
        let b = vec![0x07, 0x05, 0x01, 0x06, 0x06, 0x07];
        let nb = denybblify(b, NybbleOrder::LowFirst);
        assert_eq!(nb, vec![0x57, 0x61, 0x76]);
    }
}
