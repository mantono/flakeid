use std::{
    time::{Duration, Instant, SystemTime},
    u128,
};

#[derive(Debug)]
pub struct Flake(u128);

pub struct FlakeGen {
    node_id: u128,
    seq: u32,
}

impl FlakeGen {
    pub fn gen(&mut self) -> Result<Flake, FlakeErr> {
        let time: u128 = Self::time()?;
        let seq: u32 = self.sequence();
        let value: u128 = time ^ self.node_id ^ (seq as u128);
        Ok(Flake(value))
    }

    fn time() -> Result<u128, FlakeErr> {
        let now = SystemTime::now();
        let elapsed: Duration = match now.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(elapsed) => elapsed,
            Err(_) => return Err(FlakeErr::TimeDrift),
        };
        Ok(elapsed.as_millis())
    }

    fn sequence(&mut self) -> Option<u32> {
        self.seq.checked_add(1)
    }
}

impl Flake {
    pub fn new() -> Result<Flake, FlakeErr> {
        let time: u128 = Self::time()?;
        let addr: u128 = Self::address()?;
        let seq: u32 = Self::sequence()?;
        let value: u128 = time ^ addr ^ (seq as u128);
        Ok(Flake(value))
    }

    fn time() -> Result<u128, FlakeErr> {
        let now = SystemTime::now();
        let elapsed: Duration = match now.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(elapsed) => elapsed,
            Err(_) => return Err(FlakeErr::TimeDrift),
        };
        Ok(elapsed.as_millis())
    }

    fn address() -> Result<u128, FlakeErr> {
        Ok(0)
    }

    fn sequence() -> Result<u32, FlakeErr> {
        Ok(0)
    }
}

impl PartialEq for Flake {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
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
    #[test]
    fn two_ids_are_not_same() {
        let id1 = Flake::new().unwrap();
        let id2 = Flake::new().unwrap();
        assert_ne!(id1, id2);
    }
}
