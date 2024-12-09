use std::collections::HashSet;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct GuardMap {
    pub map: Vec<Vec<Position>>,
    pub guard: Guard,
}
#[derive(Clone)]
pub enum Position {
    Unvisited,
    Visited(HashSet<Direction>),
    Obstacle,
}
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone)]
pub struct Guard {
    pub position: (usize, usize),
    pub direction: Direction,
}

impl Guard {
    pub fn rotate(&mut self) -> &mut Self {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
        }
        self
    }
}

enum CycleResult {
    Continuable,
    Loop,
    Done,
}

impl GuardMap {
    fn cycle(&mut self) -> CycleResult {
        let next_cell_coordinates = match self.guard.direction {
            Direction::Up => {
                if self.guard.position.1 == 0 {
                    return CycleResult::Done;
                } else {
                    (self.guard.position.0, self.guard.position.1 - 1)
                }
            }
            Direction::Down => {
                if self.guard.position.1 == self.map.len() - 1 {
                    return CycleResult::Done;
                } else {
                    (self.guard.position.0, self.guard.position.1 + 1)
                }
            }
            Direction::Left => {
                if self.guard.position.0 == 0 {
                    return CycleResult::Done;
                } else {
                    (self.guard.position.0 - 1, self.guard.position.1)
                }
            }
            Direction::Right => {
                if self.guard.position.0 == self.map.get(self.guard.position.1).unwrap().len() - 1 {
                    return CycleResult::Done;
                } else {
                    (self.guard.position.0 + 1, self.guard.position.1)
                }
            }
        };
        let next_cell = self
            .map
            .get_mut(next_cell_coordinates.1)
            .unwrap()
            .get_mut(next_cell_coordinates.0)
            .unwrap();
        match next_cell {
            Position::Obstacle => {
                self.guard.rotate();
            }
            Position::Unvisited => {
                let mut set = HashSet::new();
                set.insert(self.guard.direction.clone());
                *next_cell = Position::Visited(set);
                self.guard.position = next_cell_coordinates;
            }
            Position::Visited(direction) => {
                if direction.contains(&self.guard.direction) {
                    return CycleResult::Loop;
                }
                self.guard.position = next_cell_coordinates;
            }
        }
        CycleResult::Continuable
    }
    pub fn run(&mut self) -> &mut Self {
        let mut continuable = true;
        while continuable {
            if !matches!(self.cycle(), CycleResult::Continuable) {
                continuable = false;
            }
        }
        self
    }
    pub fn count_visited(&self) -> u16 {
        self.map.iter().fold(0, |acc, line| {
            acc + line.iter().fold(0, |acc, position| match position {
                Position::Visited(_) => acc + 1,
                _ => acc,
            })
        })
    }

    pub fn count_loop_spots(&self) -> u16 {
        let mut copy_map = self.clone();
        copy_map.run();
        let mut loop_count = 0;
        for (y, line) in copy_map.map.iter().enumerate() {
            for (x, position) in line.iter().enumerate() {
                if let Position::Visited(_) = position {
                    //Skip if we're looking at the original guard position
                    if (x, y) != self.guard.position {
                        let mut guard_map_with_new_obstacle = self.clone();
                        *guard_map_with_new_obstacle
                            .map
                            .get_mut(y)
                            .unwrap()
                            .get_mut(x)
                            .unwrap() = Position::Obstacle;
                        let mut continuable = true;
                        while continuable {
                            match guard_map_with_new_obstacle.cycle() {
                                CycleResult::Continuable => {}
                                CycleResult::Loop => {
                                    loop_count += 1;
                                    continuable = false;
                                }
                                CycleResult::Done => {
                                    continuable = false;
                                }
                            }
                        }
                    }
                }
            }
        }
        loop_count
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Position::Unvisited => ".",
                Position::Visited(_) => "X",
                Position::Obstacle => "#",
            }
            .to_owned()
        )
    }
}

impl Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.direction {
                Direction::Up => "^",
                Direction::Down => "V",
                Direction::Left => "<",
                Direction::Right => ">",
            }
            .to_owned()
        )
    }
}

impl Display for GuardMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let map_str = self
            .map
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, position)| {
                        if self.guard.position == (x, y) {
                            format!("{}", self.guard)
                        } else {
                            format!("{position}")
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{map_str}")
    }
}

#[cfg(test)]
mod visited_spaces {
    use crate::day6::parser::parser;

    #[test]
    fn test_count_visited() {
        assert_eq!(parser("test_data").unwrap().run().count_visited(), 41);
    }
}
#[cfg(test)]
mod loop_spots {
    use crate::day6::parser::parser;

    #[test]
    fn test_count_loop_spots() {
        assert_eq!(parser("test_data").unwrap().count_loop_spots(), 6);
    }
}
