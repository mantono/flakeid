use data_encoding::BASE64;
use mac_address::{get_mac_address, MacAddress, MacAddressError};
use std::{
    fmt::{Binary, Display},
    time::{Duration, SystemTime},
    u128,
};

#[derive(Debug, Eq, Ord, Clone, Copy)]
pub struct Flake(u128);

impl Display for Flake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&BASE64.encode(&self.0.to_be_bytes()))
    }
}

pub struct FlakeGen {
    mac_addr: u64,
    timestamp: u128,
    seq: u16,
}

const ADDR_BITS: usize = 48;
const SEQ_BITS: usize = 16;

impl From<MacAddressError> for FlakeGenErr {
    fn from(_: MacAddressError) -> Self {
        FlakeGenErr::NoMacAddr
    }
}

impl FlakeGen {
    pub fn new() -> Result<FlakeGen, FlakeGenErr> {
        let mac_addr: MacAddress = get_mac_address()?.ok_or_else(|| FlakeGenErr::NoMacAddr)?;
        let mac_addr: u64 = mac_addr
            .bytes()
            .iter()
            .fold(0u64, |acc, value| (acc << 8) + (*value as u64));

        let gen = FlakeGen {
            mac_addr,
            timestamp: 0,
            seq: 0,
        };
        Ok(gen)
    }

    pub fn gen(&mut self) -> Result<Flake, FlakeErr> {
        let time: u128 = Self::time()?;
        let seq: u16 = self.sequence(time)?;
        let value: u128 = Self::build(time, self.mac_addr, seq);
        Ok(Flake(value))
    }

    fn build(time: u128, node: u64, seq: u16) -> u128 {
        let node: u128 = node as u128;
        let seq: u128 = seq as u128;
        let time = time << (ADDR_BITS + SEQ_BITS);
        let node = node << SEQ_BITS;

        node ^ time ^ seq
    }

    fn time() -> Result<u128, FlakeErr> {
        let now = SystemTime::now();
        let elapsed: Duration = match now.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(elapsed) => elapsed,
            Err(_) => return Err(FlakeErr::TimeDrift),
        };
        Ok(elapsed.as_millis())
    }

    fn sequence(&mut self, timestamp: u128) -> Result<u16, FlakeErr> {
        let result = match self.timestamp.cmp(&timestamp) {
            std::cmp::Ordering::Less => Ok(0),
            std::cmp::Ordering::Equal => self.seq.checked_add(1).ok_or(FlakeErr::Exhausted),
            std::cmp::Ordering::Greater => Err(FlakeErr::TimeDrift),
        };
        if let Ok(seq) = result {
            self.seq = seq;
            self.timestamp = timestamp;
        };
        result
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum FlakeGenErr {
    /// No MAC address could be found on the host device, which makes it impossible to generate
    /// flake ids that are globally unique.
    NoMacAddr,
}

impl Flake {
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

impl Binary for Flake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.0, f)
    }
}

#[derive(Debug)]
pub enum FlakeErr {
    /// A time drift (or clock skew) occured backwards in time on the host operating system.
    /// Generating new IDs while the current OS time is earlier than the time of generation for the
    /// last succesfully generated ID is not safe, since it could result in the same ID being
    /// generated twice.
    TimeDrift,
    /// The sequence number has been temporarily exhausted. This will happen if more IDs than
    /// what can be held in a u16 (65 536) is generated in a millisecond. When this occurs it is
    /// always possible to retry generating a flake id the next millisecond since that will reset
    /// the sequence counter.
    Exhausted,
}

#[cfg(test)]
mod tests {
    use crate::Flake;
    use crate::FlakeGen;
    #[test]
    fn two_ids_are_not_same() {
        let mut gen = FlakeGen::new().unwrap();
        let id1: Flake = gen.gen().unwrap();
        let id2: Flake = gen.gen().unwrap();
        println!("{} vs \n{}", id1, id2);
        println!("{:#0128b} vs \n{:#0128b}", id1, id2);
        assert_ne!(id1, id2);
    }
}
