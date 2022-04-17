// SPDX-FileCopyrightText: 2022 perillamint
//
// SPDX-License-Identifier: MIT

use rand::{thread_rng, Rng};
use std::fmt;
use std::str::FromStr;
use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH};
use uuid::Uuid;

pub struct Timeflake {
    pub timestamp: Duration,
    pub random: u128,
}

impl Timeflake {
    pub fn parse(data: &str) -> Result<Timeflake, uuid::Error> {
        // currently only support uuid-format of timeflake. Sorry!
        let uuid = Uuid::from_str(data)?;
        let flake = uuid.as_u128();

        let timestamp = Duration::from_millis(
            ((flake & 0xFFFFFFFFFFFF00000000000000000000) >> 80)
                .try_into()
                .unwrap(),
        );
        let random = flake & 0x000000000000FFFFFFFFFFFFFFFFFFFF;

        Ok(Self { timestamp, random })
    }

    pub fn random() -> Result<Timeflake, SystemTimeError> {
        Ok(Self::from_values(SystemTime::now().duration_since(UNIX_EPOCH)?, None).unwrap())
    }

    pub fn from_values(
        timestamp: Duration,
        random_val: Option<u128>,
    ) -> Result<Timeflake, rand::Error> {
        let random = match random_val {
            Some(x) => x,
            None => {
                let mut val = [0u8; 16];
                thread_rng().try_fill(&mut val[..10])?;

                u128::from_le_bytes(val)
            }
        };

        Ok(Self { timestamp, random })
    }

    pub fn get_uuid(&self) -> Uuid {
        let timestamp_part = self.timestamp.as_millis() as u64;

        let mut timeflake = self.random & 0x000000000000FFFFFFFFFFFFFFFFFFFF;

        timeflake |= (timestamp_part as u128) << 80;

        Uuid::from_u128(timeflake)
    }
}

impl fmt::Display for Timeflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_uuid())
    }
}

#[test]
fn parse_test() {
    let flake = Timeflake::from_values(Duration::from_millis(424242), Some(242424)).unwrap();
    let flake2 = Timeflake::parse(&flake.to_string()).unwrap();

    assert_eq!(flake.timestamp.as_millis(), 424242);
    assert_eq!(flake.random, 242424);
    assert_eq!(flake.timestamp, flake2.timestamp);
    assert_eq!(flake.random, flake2.random);
}

#[test]
fn exa() {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{}", Timeflake::random().unwrap());
    println!("{}", Timeflake::from_values(time, Some(0)).unwrap());
    println!("{}", Timeflake::from_values(time, None).unwrap());
    println!("{}", Timeflake::from_values(time, None).unwrap());
    println!("{}", Timeflake::from_values(time, None).unwrap());
    println!("{}", Timeflake::from_values(time, None).unwrap());
}
