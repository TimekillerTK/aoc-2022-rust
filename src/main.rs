// use aoc_2022_rust::*;
mod challenges;

use challenges::{
    day01,
    day02,
    day03
};

fn main() -> anyhow::Result<()> {
    day01();
    day02();
    day03().unwrap();

    Ok(())
}
