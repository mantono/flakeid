use mac_address::{get_mac_address, MacAddress, MacAddressError};

use crate::{id::Flake, seq::SeqGen};

pub struct FlakeGen {
    mac_addr: u64,
    seq: SeqGen,
}

const ADDR_BITS: usize = 48;
const SEQ_BITS: usize = 16;

/// TEST!
impl FlakeGen {
    /// Create a new flake ID generator. The creation may fail if it is not possible to find any
    /// device with a MAC address.
    /// ```
    /// use flakeid::id::Flake;
    /// use flakeid::gen::FlakeGen;
    /// let mut gen = FlakeGen::new().expect("Creating generator failed");
    /// let id: Flake = gen.next().expect("No ID was generated");
    /// ```
    pub fn new() -> Result<FlakeGen, FlakeGenErr> {
        let mac_addr: MacAddress = get_mac_address()?.ok_or(FlakeGenErr::NoMacAddr)?;
        let mac_addr: u64 =
            mac_addr.bytes().iter().fold(0u64, |acc, value| (acc << 8) + (*value as u64));

        let gen = FlakeGen {
            mac_addr,
            seq: SeqGen::default(),
        };
        Ok(gen)
    }

    /// Try to generate a flake ID. The generation may fail if a clock skew occurs or if
    /// the sequence number has been exhausted, but should otherwise generate an ID successfully.
    pub fn try_next(&mut self) -> Result<Flake, FlakeErr> {
        let (timestamp, seq): (u128, u16) = self.seq.try_next()?;
        let value: u128 = Self::build(timestamp, self.mac_addr, seq);
        Ok(Flake::new(value))
    }

    /// Perform the neccessary bit manipulations to transform
    /// 0000 0000 aaaa aaaa (timestamp) << 16 * 8
    /// 0000 0000 00bb bbbb (node) << 2 * 8
    /// 0000 0000 0000 00cc (seq)
    /// into                XOR
    /// aaaa aaaa bbbb bbcc
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
    /// what can be held in two bytes (65 536) is generated in a millisecond. When this occurs it is
    /// always possible to retry generating a flake ID the next millisecond since that will reset
    /// the sequence counter.
    Exhausted,
}

#[cfg(test)]
mod tests {
    use crate::gen::FlakeGen;
    use crate::id::Flake;

    #[quickcheck]
    fn test_different_timestamp_should_yield_different_value(
        ts0: u128,
        ts1: u128,
        node: u64,
        seq: u16,
    ) -> bool {
        if ts0 == ts1 {
            return true;
        }
        let id0 = FlakeGen::build(ts0, node, seq);
        let id1 = FlakeGen::build(ts1, node, seq);
        id0 != id1
    }

    #[quickcheck]
    fn test_different_node_should_yield_different_value(
        ts: u128,
        node0: u64,
        node1: u64,
        seq: u16,
    ) -> bool {
        if node0 == node1 {
            return true;
        }
        let id0 = FlakeGen::build(ts, node0, seq);
        let id1 = FlakeGen::build(ts, node1, seq);
        id0 != id1
    }

    #[quickcheck]
    fn test_different_seq_should_yield_different_value(
        ts: u128,
        node: u64,
        seq0: u16,
        seq1: u16,
    ) -> bool {
        if seq0 == seq1 {
            return true;
        }
        let id0 = FlakeGen::build(ts, node, seq0);
        let id1 = FlakeGen::build(ts, node, seq1);
        id0 != id1
    }

    #[test]
    fn two_ids_are_not_same() {
        let mut gen = FlakeGen::new().unwrap();
        let id1: Flake = gen.next().unwrap();
        let id2: Flake = gen.next().unwrap();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_first_id_less_than_second() {
        let mut gen = FlakeGen::new().unwrap();
        let id1: Flake = gen.next().unwrap();
        let id2: Flake = gen.next().unwrap();
        assert!(id1 < id2);
        assert!(id2 > id1);
    }
}
