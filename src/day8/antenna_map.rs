use std::{
    collections::{HashMap, HashSet},
    fmt,
};

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn try_new(x: i32, y: i32, max_x: usize, max_y: usize) -> Option<Self> {
        if x >= 0 && x <= max_x as i32 && y >= 0 && y <= max_y as i32 {
            Some(Coordinate::new(x as usize, y as usize))
        } else {
            None
        }
    }
    pub fn get_harmonic_antinodes(
        &self,
        point_2: &Self,
        max_x: usize,
        max_y: usize,
    ) -> HashSet<Self> {
        let mut antinodes = self.get_harmonic_antinodes_in_line(point_2, max_x, max_y);
        antinodes.extend(point_2.get_harmonic_antinodes_in_line(self, max_x, max_y));
        antinodes
    }

    fn get_harmonic_antinodes_in_line(
        &self,
        point_2: &Self,
        max_x: usize,
        max_y: usize,
    ) -> HashSet<Self> {
        let max_x = max_x as i32;
        let max_y = max_y as i32;
        let x_diff = self.x as i32 - point_2.x as i32;
        let y_diff = self.y as i32 - point_2.y as i32;
        let mut x = self.x as i32;
        let mut y = self.y as i32;
        let mut antinodes = HashSet::new();
        while x <= max_x && y <= max_y && x >= 0 && y >= 0 {
            antinodes.insert(Coordinate::new(x as usize, y as usize));
            x += x_diff;
            y += y_diff;
        }
        antinodes
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct AntennaMap {
    pub antennae: HashMap<char, HashSet<Coordinate>>,
    pub antinodes: HashSet<Coordinate>,
    pub max_x: usize,
    pub max_y: usize,
}

impl AntennaMap {
    pub fn new(antennae: HashMap<char, HashSet<Coordinate>>, max_x: usize, max_y: usize) -> Self {
        AntennaMap {
            antennae,
            antinodes: HashSet::new(),
            max_x,
            max_y,
        }
    }

    pub fn calc_antinodes(&mut self) -> &Self {
        let mut antinodes = HashSet::new();
        for (_fq, coordinates) in self.antennae.iter() {
            for coord1 in coordinates.iter() {
                // try to calculate antinodes with every other antenna of this fq
                for coord2 in coordinates.iter().filter(|coord| *coord != coord1) {
                    let (antinode1, antinode2) =
                        Self::get_antinodes(coord1, coord2, self.max_x, self.max_y);
                    if let Some(antinode) = antinode1 {
                        antinodes.insert(antinode);
                    }
                    if let Some(antinode) = antinode2 {
                        antinodes.insert(antinode);
                    }
                }
            }
        }
        self.antinodes = antinodes;
        self
    }
    pub fn calc_harmonic_antinodes(&mut self) -> &Self {
        let mut antinodes = HashSet::new();
        for (_fq, coordinates) in self.antennae.iter() {
            for coord1 in coordinates.iter() {
                // try to calculate antinodes with every other antenna of this fq
                for coord2 in coordinates.iter().filter(|coord| *coord != coord1) {
                    let new_antinodes =
                        coord1.get_harmonic_antinodes(coord2, self.max_x, self.max_y);
                    antinodes.extend(new_antinodes);
                }
            }
        }
        self.antinodes = antinodes;
        self
    }
    fn get_antinodes(
        point_1: &Coordinate,
        point_2: &Coordinate,
        max_x: usize,
        max_y: usize,
    ) -> (Option<Coordinate>, Option<Coordinate>) {
        let x_diff = point_1.x as i32 - point_2.x as i32;
        let y_diff = point_1.y as i32 - point_2.y as i32;
        let possible_antinode_1_x = point_1.x as i32 + x_diff;
        let possible_antinode_1_y = point_1.y as i32 + y_diff;
        let possible_antinode_2_x = point_2.x as i32 - x_diff;
        let possible_antinode_2_y = point_2.y as i32 - y_diff;
        let antinode_1 =
            Coordinate::try_new(possible_antinode_1_x, possible_antinode_1_y, max_x, max_y);
        let antinode_2 =
            Coordinate::try_new(possible_antinode_2_x, possible_antinode_2_y, max_x, max_y);
        (antinode_1, antinode_2)
    }
    pub fn count_antinodes(&self) -> usize {
        self.antinodes.len()
    }
}

impl fmt::Display for AntennaMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = (0..self.max_y + 1)
            .map(|y| {
                (0..self.max_x + 1)
                    .map(|x| {
                        let coord = Coordinate::new(x, y);
                        if self.antinodes.contains(&coord) {
                            '#'
                        } else if let Some((fq, _set)) =
                            self.antennae.iter().find(|(_fq, set)| set.contains(&coord))
                        {
                            *fq
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod test_coord {
    // just testing to make sure this works right
    use super::Coordinate;

    #[test]
    fn test_coordinate_eq() {
        assert_eq!(Coordinate::new(1, 2), Coordinate::new(1, 2));
        assert_ne!(Coordinate::new(1, 2), Coordinate::new(1, 3));
    }
}

#[cfg(test)]
mod test_antenna_map {
    use std::collections::{HashMap, HashSet};

    // just testing to make sure this works right
    use super::{AntennaMap, Coordinate};
    use crate::day8::parser::parser;

    #[test]
    fn test_display() {
        //expected
        let expected_antinodeless = "............\n........0...\n.....0......\n.......0....\n....0.......\n......A.....\n............\n............\n........A...\n.........A..\n............\n............".to_owned();
        let expected_with_antinodes = "......#....#\n...#....0...\n....#0....#.\n..#....0....\n....0....#..\n.#....#.....\n...#........\n#......#....\n........A...\n.........A..\n..........#.\n..........#.".to_owned();

        //given
        let mut antenna_map = parser("test_input").unwrap();

        //then
        assert_eq!(format!("{antenna_map}"), expected_antinodeless);
        antenna_map.calc_antinodes();
        println!("{antenna_map}");
        assert_eq!(format!("{antenna_map}"), expected_with_antinodes);
    }
    #[test]
    fn test_calc_antinodes() {
        //expected
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

        let mut expected_antinodes = HashSet::new();
        expected_antinodes.insert(Coordinate::new(3, 6));
        expected_antinodes.insert(Coordinate::new(1, 5));
        expected_antinodes.insert(Coordinate::new(10, 11));
        expected_antinodes.insert(Coordinate::new(6, 0));
        expected_antinodes.insert(Coordinate::new(11, 0));
        expected_antinodes.insert(Coordinate::new(3, 1));
        expected_antinodes.insert(Coordinate::new(0, 7));
        expected_antinodes.insert(Coordinate::new(2, 3));
        expected_antinodes.insert(Coordinate::new(4, 2));
        expected_antinodes.insert(Coordinate::new(10, 2));
        expected_antinodes.insert(Coordinate::new(6, 5));
        expected_antinodes.insert(Coordinate::new(10, 10));
        expected_antinodes.insert(Coordinate::new(9, 4));
        expected_antinodes.insert(Coordinate::new(7, 7));
        let mut expected_antenna_map = AntennaMap::new(expected_antennae, 11, 11);
        expected_antenna_map.antinodes = expected_antinodes;

        //given
        let mut antenna_map = parser("test_input").unwrap();

        //then
        assert_eq!(*antenna_map.calc_antinodes(), expected_antenna_map);
        assert_eq!(antenna_map.count_antinodes(), 14);
    }
    #[test]
    fn test_calc_harmonic_antinodes() {
        //given
        let mut antenna_map = parser("test_input").unwrap();

        //then
        let _harmonic_antinodes = antenna_map.calc_harmonic_antinodes();
        assert_eq!(antenna_map.count_antinodes(), 34);
    }
}
