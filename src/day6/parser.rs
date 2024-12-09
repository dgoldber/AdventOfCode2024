use crate::day6::guard_map::*;

use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn parser(file_name: &str) -> io::Result<GuardMap> {
    let root = "./src/day6";
    let file = File::open(format!("{root}/{file_name}"))?;
    let reader = BufReader::new(file);
    let mut guard = None;
    let map = reader
        .lines()
        .map_while(Result::ok)
        .enumerate()
        .map(|(y, line)| {
            // y starts from the top and goes downward
            line.char_indices()
                .map(|(x, value)| {
                    let parsed = parse_char(value)?;
                    if let Some(direction) = parsed.1 {
                        guard = Some(Guard {
                            position: (x, y),
                            direction,
                        });
                    }
                    Ok::<Position, ParsePositionError>(parsed.0)
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("Could not parse grid");

    Ok(GuardMap {
        map,
        guard: guard.expect("Could not find guard"),
    })
}

// Returns a position, and if there's a guard there, his direction. Throws a ParsePositionError for invalid characters
fn parse_char(value: char) -> Result<(Position, Option<Direction>), ParsePositionError> {
    match value {
        '.' => Ok((Position::Unvisited, None)),
        '#' => Ok((Position::Obstacle, None)),
        '^' => {
            let mut set = HashSet::new();
            set.insert(Direction::Up);
            Ok((Position::Visited(set), Some(Direction::Up)))
        }
        'V' => {
            let mut set = HashSet::new();
            set.insert(Direction::Up);
            Ok((Position::Visited(set), Some(Direction::Down)))
        }
        '<' => {
            let mut set = HashSet::new();
            set.insert(Direction::Up);
            Ok((Position::Visited(set), Some(Direction::Left)))
        }
        '>' => {
            let mut set = HashSet::new();
            set.insert(Direction::Up);
            Ok((Position::Visited(set), Some(Direction::Right)))
        }
        _ => Err(ParsePositionError::new(value)),
    }
}

struct ParsePositionError {
    input: char,
}

impl ParsePositionError {
    pub fn new(input: char) -> ParsePositionError {
        Self { input }
    }
}

impl std::fmt::Display for ParsePositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse {} to a Position", self.input)
    }
}

impl fmt::Debug for ParsePositionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

#[cfg(test)]
mod sort_update {

    use super::parser;

    #[test]
    fn test_parser() {
        let guard_map = parser("test_data").unwrap();
        assert_eq!(
            format!("{guard_map}"),
            format!("....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...")
        );
    }
}
