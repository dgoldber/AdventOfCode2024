use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use crate::day8::antenna_map::AntennaMap;

use super::antenna_map::Coordinate;

pub fn parser(file_name: &str) -> io::Result<AntennaMap> {
    let root = "./src/day8";
    let file = File::open(format!("{root}/{file_name}"))?;
    let reader = BufReader::new(file);
    let mut antennae: HashMap<char, HashSet<Coordinate>> = HashMap::new();
    // let mut antenna_map = AntennaMap::new();
    let mut max_y = 0;
    let mut max_x = 0;
    for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
        for (x, value) in line
            .char_indices()
            .filter(|(_, val)| val.is_ascii_alphanumeric())
        {
            if let Some(antennae_of_this_frequency) = antennae.get_mut(&value) {
                antennae_of_this_frequency.insert(Coordinate::new(x, y));
            } else {
                let mut new_set = HashSet::new();
                new_set.insert(Coordinate::new(x, y));
                antennae.insert(value, new_set);
            }
        }
        if max_x == 0 {
            max_x = line.len() - 1;
        }
        max_y = y;
    }
    Ok(AntennaMap::new(antennae, max_x, max_y))
}

#[cfg(test)]
mod test_parser {

    use std::collections::{HashMap, HashSet};

    use super::parser;
    use crate::day8::antenna_map::{AntennaMap, Coordinate};

    #[test]
    fn test_parser() {
        let antenna_map = parser("test_input").unwrap();
        let mut expected_antennae = HashMap::new();
        let mut set_0: HashSet<Coordinate> = HashSet::new();
        set_0.insert(Coordinate::new(5, 2));
        set_0.insert(Coordinate::new(8, 1));
        set_0.insert(Coordinate::new(7, 3));
        set_0.insert(Coordinate::new(4, 4));
        expected_antennae.insert('0', set_0);
        let mut set_a: HashSet<Coordinate> = HashSet::new();
        set_a.insert(Coordinate::new(8, 8));
        set_a.insert(Coordinate::new(6, 5));
        set_a.insert(Coordinate::new(9, 9));
        expected_antennae.insert('A', set_a);
        assert_eq!(antenna_map, AntennaMap::new(expected_antennae, 11, 11));
    }
}
