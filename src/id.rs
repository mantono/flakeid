use core::hash::Hash;
use data_encoding::BASE64;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use std::{
    fmt::{Binary, Display},
    u128,
};

#[derive(Debug, Eq, Clone, Copy)]
pub struct Flake(u128);

impl Flake {
    pub(crate) fn new(value: u128) -> Flake {
        Self(value)
    }

    pub fn value(&self) -> u128 {
        self.0
    }

    /// Byte array representation of the Flake ID. Endianness is always little-endianness so byte
    /// representation is consistent across different platforms.
    #[inline(always)]
    pub fn bytes(&self) -> [u8; 16] {
        self.0.to_le_bytes()
    }

    /// Creates a flake id from an array of 16 bytes. Endianness of the byte array is assumed to be
    /// little endianess.
    pub fn from_bytes(bytes: [u8; 16]) -> Flake {
        Flake::new(u128::from_le_bytes(bytes))
    }

    /// Returns a timestamp in form of number of **milliseconds** since UNIX epoch time
    /// (1st of January 1970 UTC).
    pub fn timestamp(&self) -> u64 {
        let ts: u128 = self.0 >> 64;
        u64::try_from(ts).expect("Timestamp must fit into an usigned 64 bit integer")
    }
}

impl PartialEq for Flake {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Flake {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Ord for Flake {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Hash for Flake {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u128(self.0)
    }
}

impl Binary for Flake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.0, f)
    }
}

impl Display for Flake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&BASE64.encode(&self.0.to_be_bytes()))
    }
}

impl FromStr for Flake {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: String = s.to_string();
        let bytes = s.as_bytes();
        let decoded: Vec<u8> = match &BASE64.decode(bytes) {
            Ok(vec) => vec.clone(),
            Err(e) => match e.kind {
                data_encoding::DecodeKind::Length => todo!(),
                data_encoding::DecodeKind::Symbol => todo!(),
                data_encoding::DecodeKind::Trailing => todo!(),
                data_encoding::DecodeKind::Padding => todo!(),
            },
        };
        let bytes: [u8; 16] = match decoded.try_into() {
            Ok(arr) => arr,
            Err(_) => todo!("Unable to convert slice to array"),
        };
        Ok(Self::from_bytes(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::Flake;

    #[test]
    fn test_byte_repr() {
        let id0 = Flake::new(29866156537351941961353716432896);
        let bytes = id0.bytes();
        let id1 = Flake::from_bytes(bytes);
        assert_eq!(id0, id1);
    }

    #[test]
    fn test_from_str() {
        let id0 = Flake::new(29866156537351941961353716432896);
        let string_flake: String = id0.to_string();
        let id1: Flake = string_flake.parse().unwrap();
        assert_eq!(id0, id1);
    }

    #[test]
    fn test_timestamp() {}
}
