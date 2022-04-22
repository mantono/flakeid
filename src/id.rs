use core::hash::Hash;
use data_encoding::BASE64;
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

    /// Byte array representation of the Flake ID, in native byte order
    #[inline(always)]
    pub fn bytes(&self) -> [u8; 16] {
        self.0.to_ne_bytes()
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
