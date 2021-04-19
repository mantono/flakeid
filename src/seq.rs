use std::time::{Duration, SystemTime};

use crate::gen::FlakeErr;

pub(crate) struct SeqGen {
    timestamp: u128,
    seq: u16,
}

impl Default for SeqGen {
    fn default() -> Self {
        SeqGen {
            timestamp: 0,
            seq: 0,
        }
    }
}

impl SeqGen {
    pub fn try_next(&mut self) -> Result<(u128, u16), FlakeErr> {
        let now: u128 = Self::time()?;
        let seq: u16 = match self.timestamp.cmp(&now) {
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Equal => match self.seq.checked_add(1) {
                Some(n) => n,
                None => return Err(FlakeErr::Exhausted),
            },
            std::cmp::Ordering::Greater => return Err(FlakeErr::TimeDrift),
        };
        self.timestamp = now;
        self.seq = seq;
        Ok((now, seq))
    }

    fn time() -> Result<u128, FlakeErr> {
        let now = SystemTime::now();
        let elapsed: Duration = match now.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(elapsed) => elapsed,
            Err(_) => return Err(FlakeErr::TimeDrift),
        };
        Ok(elapsed.as_millis())
    }
}
