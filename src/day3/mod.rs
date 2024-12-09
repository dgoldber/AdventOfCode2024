use regex::Regex;
use std::fs::File;
use std::io::{self, BufReader, Read};

pub fn day3() -> io::Result<()> {
    let mut data: BufReader<File> = BufReader::new(File::open("./src/day3/input.txt")?);
    let mut input_str = String::new();
    data.read_to_string(&mut input_str)?;
    println!(
        "part 1: {}",
        MultParser::new(input_str.clone())
            .parse_simple()
            .calculate_total()
    );
    println!(
        "part 2: {}",
        MultParser::new(input_str).parse().calculate_total()
    );
    Ok(())
}

struct MultParser {
    input: String,
    regex: Regex,
}

impl MultParser {
    pub fn new(input: String) -> Self {
        MultParser {
            input,
            regex: Regex::new(
                r"mul\((?<x>[0-9]|([1-9][0-9]{1,2})),(?<y>[0-9]|([1-9][0-9]{1,2}))\)",
            )
            .unwrap(),
        }
    }
    fn parse_to_vec(&self, input: &str) -> Vec<(u16, u16)> {
        self.regex
            .captures_iter(input)
            .map(|captures| {
                (
                    captures["x"].to_owned().parse::<u16>().unwrap(),
                    captures["y"].to_owned().parse::<u16>().unwrap(),
                )
            })
            .collect::<Vec<(u16, u16)>>()
    }
    pub fn parse_simple(&self) -> ParsedMult {
        ParsedMult {
            data: self.parse_to_vec(&self.input),
        }
    }
    pub fn parse(&self) -> ParsedMult {
        ParsedMult {
            data: self
                .input
                .split("don't()")
                .enumerate()
                .flat_map(|(index, section_beginning_with_dont)| {
                    if index == 0 {
                        // If this is the first section, it doesn't actually begin with `don't()`, so we can just parse it
                        self.parse_to_vec(section_beginning_with_dont)
                    } else {
                        section_beginning_with_dont
                            .split("do()")
                            .enumerate()
                            .filter_map(|(index, subsection_beginning_with_do)| {
                                if index == 0 {
                                    // Similarly, if this is the first subsection, it doesn't actually begin with do(), so we filter it out
                                    None
                                } else {
                                    Some(self.parse_to_vec(subsection_beginning_with_do))
                                }
                            })
                            .flatten()
                            .collect::<Vec<_>>()
                    }
                })
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct ParsedMult {
    pub data: Vec<(u16, u16)>,
}

impl ParsedMult {
    pub fn calculate_total(&self) -> u32 {
        self.data
            .iter()
            .fold(0, |acc, tup| acc + (u32::from(tup.0) * u32::from(tup.1)))
    }
}
