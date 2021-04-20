use mac_address::{get_mac_address, MacAddress, MacAddressError};

use crate::{id::Flake, seq::SeqGen};

pub struct FlakeGen {
    mac_addr: u64,
    seq: SeqGen,
}

const ADDR_BITS: usize = 48;
const SEQ_BITS: usize = 16;

impl FlakeGen {
    /// Create a new Flake ID generator. The creation may fail if it is not possible to find any
    /// device with a MAC address.
    /// ```
    /// use flakeid::id::Flake;
    /// use flakeid::gen::FlakeGen;
    /// let mut gen = FlakeGen::new().expect("Creating generator failed");
    /// let id: Flake = gen.next().expect("No ID was generated");
    /// ```
    pub fn new() -> Result<FlakeGen, FlakeGenErr> {
        let mac_addr: MacAddress = get_mac_address()?.ok_or(FlakeGenErr::NoMacAddr)?;
        let mac_addr: u64 = mac_addr
            .bytes()
            .iter()
            .fold(0u64, |acc, value| (acc << 8) + (*value as u64));

        let gen = FlakeGen {
            mac_addr,
            seq: SeqGen::default(),
        };
        Ok(gen)
    }

    pub fn try_next(&mut self) -> Result<Flake, FlakeErr> {
        let (timestamp, seq): (u128, u16) = self.seq.try_next()?;
        let value: u128 = Self::build(timestamp, self.mac_addr, seq);
        Ok(Flake::new(value))
    }

    fn build(time: u128, node: u64, seq: u16) -> u128 {
        let node: u128 = node as u128;
        let seq: u128 = seq as u128;
        let time = time << (ADDR_BITS + SEQ_BITS);
        let node = node << SEQ_BITS;

        node ^ time ^ seq
    }
}

impl Iterator for FlakeGen {
    type Item = Flake;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().ok()
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum FlakeGenErr {
    /// No MAC address could be found on the host device, which makes it impossible to generate
    /// flake ids that are globally unique.
    NoMacAddr,
}

impl From<MacAddressError> for FlakeGenErr {
    fn from(_: MacAddressError) -> Self {
        FlakeGenErr::NoMacAddr
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
