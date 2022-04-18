// SPDX-FileCopyrightText: 2022 perillamint
//
// SPDX-License-Identifier: MIT

use rand::{thread_rng, Rng};
use std::fmt;
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

mod error;

use error::TimeflakeError;

pub struct Timeflake {
    pub timestamp: Duration,
    pub random: u128,
}

impl Timeflake {
    pub fn parse(data: &str) -> Result<Timeflake, TimeflakeError> {
        // currently only support uuid-format of timeflake. Sorry!
        let uuid = match Uuid::from_str(data) {
            Ok(x) => Ok(x),
            Err(e) => Err(TimeflakeError::MalformedData { msg: e.to_string() }),
        }?;

        let flake = uuid.as_u128();

        let timestamp = Duration::from_millis(
            // If this fails, something is terribly wrong anyway.
            ((flake & 0xFFFFFFFFFFFF00000000000000000000) >> 80)
                .try_into()
                .unwrap(),
        );
        let random = flake & 0x000000000000FFFFFFFFFFFFFFFFFFFF;

        Ok(Self { timestamp, random })
    }

    pub fn random() -> Result<Timeflake, TimeflakeError> {
        let time = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(x) => x,
            Err(e) => return Err(TimeflakeError::SystemTimeError { msg: e.to_string() }),
        };

        Self::from_values(time, None)
    }

    pub fn from_values(
        timestamp: Duration,
        random_val: Option<u128>,
    ) -> Result<Timeflake, TimeflakeError> {
        let random = match random_val {
            Some(x) => x,
            None => {
                let mut val = [0u8; 16];
                match thread_rng().try_fill(&mut val[..10]) {
                    Ok(_) => {}
                    Err(e) => return Err(TimeflakeError::RNGError { msg: e.to_string() }),
                }

                u128::from_le_bytes(val)
            }
        };

        Ok(Self { timestamp, random })
    }

    pub fn as_u128(&self) -> u128 {
        let timeflake = self.random & 0x000000000000FFFFFFFFFFFFFFFFFFFF;
        let timestamp_part = self.timestamp.as_millis() as u64;
        timeflake | (timestamp_part as u128) << 80
    }

    pub fn get_uuid(&self) -> Uuid {
        Uuid::from_u128(self.as_u128())
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
fn example() {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{}", Timeflake::random().unwrap());
    println!("{}", Timeflake::from_values(time, Some(0)).unwrap());
    println!("{}", Timeflake::from_values(time, None).unwrap());
    println!("{}", Timeflake::from_values(time, None).unwrap());
}
