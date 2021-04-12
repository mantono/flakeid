use std::{
    fmt::{Display, Write},
    time::{Duration, Instant, SystemTime},
    u128,
};

use data_encoding::BASE64;

#[derive(Debug, Eq, Ord, Clone, Copy)]
pub struct Flake(u128);

impl Display for Flake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&BASE64.encode(&self.0.to_le_bytes()))
    }
}

pub struct FlakeGen {
    node_id: u128,
    timestamp: u128,
    seq: u16,
}

const TIME_BITS: usize = 64;
const NODE_BITS: usize = 48;
const SEQ_BITS: usize = 16;

impl FlakeGen {
    pub fn new(node_id: Option<u128>) -> FlakeGen {
        FlakeGen {
            node_id: node_id.unwrap_or(0),
            timestamp: 0,
            seq: 0,
        }
    }

    pub fn gen(&mut self) -> Result<Flake, FlakeErr> {
        let time: u128 = Self::time()?;
        let seq: u16 = self.sequence(time)?;
        let value: u128 = Self::build(time, self.node_id, seq);
        Ok(Flake(value))
    }

    /// |                              Time                                     |                       Node                          |       Seq       |
    ///  00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
    fn build(time: u128, node: u128, seq: u16) -> u128 {
        println!("{}.{}.{}", time, node, seq);
        let time = time << (NODE_BITS + SEQ_BITS);
        let node = (node << (NODE_BITS + SEQ_BITS)) >> TIME_BITS;
        println!("{}.{}.{}", time, node, seq);
        dbg!(node ^ time ^ (seq as u128))
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
            self.seq = dbg!(seq);
            self.timestamp = dbg!(timestamp);
        };
        result
    }
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

#[derive(Debug)]
pub enum FlakeErr {
    TimeDrift,
    Exhausted,
}

#[cfg(test)]
mod tests {
    use crate::Flake;
    use crate::FlakeGen;
    #[test]
    fn two_ids_are_not_same() {
        let mut gen = FlakeGen::new(None);
        let id1 = gen.gen().unwrap();
        let id2 = gen.gen().unwrap();
        println!("{} vs {}", id1, id2);
        assert_ne!(id1, id2);
    }
}
