use core::hash::Hash;
use data_encoding::BASE64;
use std::convert::TryFrom;
use std::fmt::{Binary, Display};
use std::fmt::{LowerHex, UpperHex};

/// A Flake identifier (ID) is an identifier which consists of three different parts:
/// 1. Timestamp (in milliseconds from the [UNIX epoch](https://en.wikipedia.org/wiki/Unix_time))
/// 2. "Node" identifier, usually a MAC address
/// 3. Sequence number
///
/// These three parts together will make up an identifier, which is time ordered. This means that if flake identifier _a_ is created before flake identifier _b_, then _a_ is lexically ordered before _b_.
///
/// ```
/// # use flakeid::id::Flake;
/// # use flakeid::gen::FlakeGen;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut gen = FlakeGen::new(0xC0FFEE);
/// let a: Flake = gen.try_next()?;
/// let b: Flake = gen.try_next()?;
/// assert!(a < b);
/// # Ok(())
/// # }
/// ```
///
/// Internally, the flake identifier is represented as a `u128`, where the first 64 bits are the
/// timestamp, the next 48 bits are the node identifier and the last 18 bits are the sequence
/// number.
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

    /// Returns a timestamp in form of number of **milliseconds** since UNIX epoch time
    /// (1st of January 1970 UTC).
    pub fn timestamp(&self) -> u64 {
        let ts: u128 = self.0 >> 64;
        u64::try_from(ts).expect("Timestamp must fit into an usigned 64 bit integer")
    }
}

impl From<[u8; 16]> for Flake {
    /// Creates a flake id from an array of 16 bytes. Endianness of the byte array is assumed to be
    /// little endianess.
    fn from(value: [u8; 16]) -> Self {
        Flake::new(u128::from_le_bytes(value))
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

impl LowerHex for Flake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&self.value(), f)
    }
}

impl UpperHex for Flake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        UpperHex::fmt(&self.value(), f)
    }
}

impl Display for Flake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&BASE64.encode(&self.0.to_be_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use super::Flake;

    #[test]
    fn test_byte_repr() {
        let id0 = Flake::new(29866156537351941961353716432896);
        let bytes = id0.bytes();
        let id1: Flake = bytes.into();
        assert_eq!(id0, id1);
    }

    #[test]
    fn test_timestamp() {
        let id = Flake::new(30556157387769903979283677052928);
        let ts: u64 = id.timestamp();
        assert_eq!(1656452611131, ts);
    }
}
