use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn day4() -> io::Result<()> {
    let grid = parser("input.txt")?;
    println!("Part1: {}", WordSearcher::from(grid.clone()).search("XMAS"));
    println!("Part2: {}", WordSearcher::from(grid).search_mas());
    Ok(())
}

fn parser(file_name: &str) -> io::Result<Vec<Vec<char>>> {
    let root = "./src/day4";
    let file = File::open(format!("{root}/{file_name}"))?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>())
}

struct WordSearcher {
    grid: Vec<Vec<char>>,
}

impl From<Vec<Vec<char>>> for WordSearcher {
    fn from(grid: Vec<Vec<char>>) -> Self {
        WordSearcher { grid }
    }
}
#[derive(Debug)]
enum Direction {
    Asc,
    Desc,
    None,
}

impl Direction {
    fn get_index(&self, grid_index: usize, word_index: usize) -> usize {
        match self {
            Self::None => grid_index,
            Self::Asc => grid_index + (word_index + 1),
            Self::Desc => grid_index - (word_index + 1),
        }
    }
}
impl WordSearcher {
    //This will panic if the word is an empty string
    pub fn search(&self, word: &str) -> u16 {
        let mut total = 0;
        for (y, line) in self.grid.iter().enumerate() {
            for (x, letter) in line.iter().enumerate() {
                total += self.part1(x, y, letter, line, word);
            }
        }
        total
    }
    pub fn search_mas(&self) -> u16 {
        let mut total = 0;
        for (y, line) in self.grid.iter().enumerate() {
            for (x, letter) in line.iter().enumerate() {
                // My abstractions didn't help, so I'm hardcoding more things here because this is taking too long

                // Don't need to check the outer edge
                if x > 0 && y > 0 && x < line.len() - 1 && y < self.grid.len() - 1 && letter == &'A'
                {
                    let top_left = self.grid.get(y - 1).unwrap().get(x - 1).unwrap();
                    let bottom_right = self.grid.get(y + 1).unwrap().get(x + 1).unwrap();

                    let top_right = self.grid.get(y - 1).unwrap().get(x + 1).unwrap();
                    let bottom_left = self.grid.get(y + 1).unwrap().get(x - 1).unwrap();

                    if (*top_left == 'M' && *bottom_right == 'S'
                        || *top_left == 'S' && *bottom_right == 'M')
                        && (*top_right == 'M' && *bottom_left == 'S'
                            || *top_right == 'S' && *bottom_left == 'M')
                    {
                        total += 1
                    }
                }
            }
        }
        total
    }

    fn part1(&self, x: usize, y: usize, letter: &char, line: &[char], word: &str) -> u16 {
        let mut total = 0;
        //If the letter is the first character in the string we're looking for
        if letter == &word.chars().next().unwrap() {
            let room_above = y <= self.grid.len() - word.len();
            let room_below = y >= word.len() - 1;
            let room_right = x <= line.len() - word.len();
            let room_left = x >= word.len() - 1;
            if room_above && self.check(word, x, y, Direction::None, Direction::Asc) {
                total += 1;
            }
            if room_above && room_right && self.check(word, x, y, Direction::Asc, Direction::Asc) {
                total += 1;
            }
            if room_right && self.check(word, x, y, Direction::Asc, Direction::None) {
                total += 1;
            }
            if room_right && room_below && self.check(word, x, y, Direction::Asc, Direction::Desc) {
                total += 1;
            }
            if room_below && self.check(word, x, y, Direction::None, Direction::Desc) {
                total += 1;
            }
            if room_below && room_left && self.check(word, x, y, Direction::Desc, Direction::Desc) {
                total += 1;
            }
            if room_left && self.check(word, x, y, Direction::Desc, Direction::None) {
                total += 1;
            }
            if room_left && room_above && self.check(word, x, y, Direction::Desc, Direction::Asc) {
                total += 1;
            }
        }
        total
    }

    fn unsafe_check(&self, x: usize, y: usize, needle: char) -> bool {
        unsafe { self.grid.get_unchecked(y).get_unchecked(x) == &needle }
    }
    fn check(
        &self,
        word: &str,
        x: usize,
        y: usize,
        x_direction: Direction,
        y_direction: Direction,
    ) -> bool {
        word.split_at(1).1.char_indices().all(|(index, letter)| {
            self.unsafe_check(
                x_direction.get_index(x, index),
                y_direction.get_index(y, index),
                letter,
            )
        })
    }
}

#[cfg(test)]
mod word_searcher {
    use crate::day4::parser;

    use super::WordSearcher;

    #[test]
    fn test_simple() {
        assert_eq!(
            WordSearcher::from(vec![
                vec!['.', '.', 'X', '.', '.', '.'],
                vec!['.', 'S', 'A', 'M', 'X', '.'],
                vec!['.', 'A', '.', '.', 'A', '.'],
                vec!['X', 'M', 'A', 'S', '.', 'S'],
                vec!['.', 'X', '.', '.', '.', '.']
            ])
            .search("XMAS"),
            4
        );
    }

    #[test]
    fn test_mas() {
        assert_eq!(
            WordSearcher::from(parser("test_data").unwrap()).search_mas(),
            9
        );
    }
    #[test]
    fn test_larger() {
        assert_eq!(
            WordSearcher::from(vec![
                "MMMSXXMASM".chars().collect(),
                "MSAMXMSMSA".chars().collect(),
                "AMXSXMAAMM".chars().collect(),
                "MSAMASMSMX".chars().collect(),
                "XMASAMXAMM".chars().collect(),
                "XXAMMXXAMA".chars().collect(),
                "SMSMSASXSS".chars().collect(),
                "SAXAMASAAA".chars().collect(),
                "MAMMMXMMMM".chars().collect(),
                "MXMXAXMASX".chars().collect(),
            ])
            .search("XMAS"),
            18
        );
    }
}
