use crate::day7::{equation::Equation, parser::parser};
use std::io;

mod equation;
mod parser;

pub fn day7() -> io::Result<()> {
    println!(
        "Part2: {}",
        Equation::get_total_calibration_results(parser("input")?)
    );
    Ok(())
}
