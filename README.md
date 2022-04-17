<!--
SPDX-FileCopyrightText: 2022 perillamint

SPDX-License-Identifier: CC0-1.0
-->

# Timeflake-rs
[![Build Status](https://github.com/perillamint/timeflake-rs/workflows/CI/badge.svg)](https://github.com/perillamint/timeflake-rs/actions)
[![crates.io](https://img.shields.io/crates/v/timeflake-rs.svg)](https://crates.io/crates/timeflake-rs)
[![License](https://img.shields.io/github/license/perillamint/timeflake-rs)](https://github.com/perillamint/timeflake-rs/blob/master/LICENSES/MIT.txt)

Timeflake is a 128-bit, roughly-ordered, URL-safe UUID. Inspired by Twitter's Snowflake, Instagram's ID and Firebase's PushID.

Port of [https://github.com/anthonynsimon/timeflake](https://github.com/anthonynsimon/timeflake) in pure Rust

# Example code
```
use Timeflake;

fn main() {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{}", Timeflake::random().unwrap());
    println!("{}", Timeflake::from_values(time, Some(0)).unwrap());
    println!("{}", Timeflake::from_values(time, None).unwrap());
}
```