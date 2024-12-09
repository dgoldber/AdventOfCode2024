use crate::day8::parser::parser;
use std::io;

mod antenna_map;
mod parser;

pub fn day8() -> io::Result<()> {
    println!(
        "Part1: {}",
        (parser("input")?.calc_antinodes().count_antinodes())
    );
    println!(
        "Part2: {}",
        (parser("input")?.calc_harmonic_antinodes().count_antinodes())
    );
    Ok(())
}
