use crate::day6::parser::*;
use std::io::{self};

mod guard_map;
mod parser;

pub fn day6() -> io::Result<()> {
    println!("Part1: {}", parser("input")?.run().count_visited());
    println!("Part2: {}", parser("input")?.count_loop_spots());
    Ok(())
}
