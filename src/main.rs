#![allow(dead_code, unused_imports, unused_variables)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]
#![forbid(unsafe_code)]

mod days;
mod error;
mod prelude;
mod util;

use crate::prelude::*;

fn main() -> Aoc2021Result<()> {
    println!("|----------------------------|");
    println!("| Advent of Code 2021 ~ Rust |");
    println!("|----------------------------|");

    report_day02()?;

    Ok(())
}
