use core::hash::Hash;
use data_encoding::BASE64;
use std::{
    fmt::{Binary, Display},
    u128,
};

#[derive(Debug, Eq, Clone, Copy)]
pub struct Flake(u128);

impl Display for Flake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&BASE64.encode(&self.0.to_be_bytes()))
    }
}

impl Flake {
    pub(crate) fn new(value: u128) -> Flake {
        Self(value)
    }

    pub fn value(&self) -> u128 {
        self.0
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

#[cfg(test)]
mod tests {
    use crate::gen::FlakeGen;
    use crate::id::Flake;
    #[test]
    fn two_ids_are_not_same() {
        let mut gen = FlakeGen::new().unwrap();
        let id1: Flake = gen.next().unwrap();
        let id2: Flake = gen.next().unwrap();
        println!("{} vs \n{}", id1, id2);
        println!("{:#0128b} vs \n{:#0128b}", id1, id2);
        assert_ne!(id1, id2);
    }
}
