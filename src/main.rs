use std::io::{self, stdin, Write};
mod day1;
mod day2;
mod day3;

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
            day2::day2();
        }
        Ok(3) => {
            day3::day3();
        }
        _ => {
            println!("No");
        }
    }
    Ok(())
}
