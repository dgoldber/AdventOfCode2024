use std::io::{self, stdin, Write};
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() -> io::Result<()> {
    print!("Enter a day: ");
    io::stdout().flush()?;
    let mut buffer = String::new();
    let stdin = stdin();
    stdin.read_line(&mut buffer)?;

    match buffer.trim().parse::<u8>() {
        Ok(1) => {
            day1::day1()?;
        }
        Ok(2) => {
            day2::day2()?;
        }
        Ok(3) => {
            day3::day3()?;
        }
        Ok(4) => {
            day4::day4()?;
        }
        Ok(5) => {
            day5::day5()?;
        }
        Ok(6) => {
            day6::day6()?;
        }
        Ok(7) => {
            day7::day7()?;
        }
        Ok(8) => {
            day8::day8()?;
        }
        _ => {
            println!("No");
        }
    }
    Ok(())
}
