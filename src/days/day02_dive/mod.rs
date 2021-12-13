use crate::prelude::*;

mod coordinate;
mod direction;
mod movement;
mod submarine;

pub(crate) use coordinate::*;
pub(crate) use direction::*;
pub(crate) use movement::*;
pub(crate) use submarine::*;

pub(crate) fn report_day02() -> Aoc2021Result<()> {
    println!("-- Day 02: Dive! --");
    let location_code = Submarine::default()
        .read_instructions(input_str(2)?.as_str())?
        .location_code();

    println!("Part 1: '{}', Part 2: '{:?}'", location_code, ());

    Ok(())
}
