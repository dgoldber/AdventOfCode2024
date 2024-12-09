use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn day5() -> io::Result<()> {
    println!(
        "Part1: {}",
        parser("./src/day5/input")?.get_valid_middle_total()
    );
    println!(
        "Part2: Not working {}",
        parser("./src/day5/input")?.get_corrected_middle_total()
    );
    Ok(())
}

fn parser(file_path: &str) -> io::Result<Updater> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut rules: Vec<(u16, u16)> = vec![];
    let mut updates: Vec<Vec<u16>> = vec![];
    let mut parsing_rules = true;
    for line in reader.lines().map_while(Result::ok) {
        if parsing_rules {
            if line.is_empty() {
                parsing_rules = false;
                continue;
            }
            let mut split = line.split('|');
            rules.push((
                split
                    .next()
                    .expect("rule part 1 was not found")
                    .parse()
                    .expect("rule part 1 was not a number"),
                split
                    .next()
                    .expect("rule part 2 was not found")
                    .parse()
                    .expect("rule part 2 was not a number"),
            ));
        } else {
            updates.push(
                line.split(',')
                    .map(|elem| elem.parse().expect("Update included a non number"))
                    .collect(),
            );
        }
    }
    // println!("Rules: {:?}", rules);
    // println!("Updates: {:?}", updates);
    Ok(Updater { rules, updates })
}

#[derive(PartialEq, Eq, Debug)]
struct Updater {
    rules: Vec<(u16, u16)>,
    updates: Vec<Vec<u16>>,
}

impl Updater {
    pub fn get_valid_middle_total(&self) -> u16 {
        self.updates.clone().iter().fold(0, |acc, update| {
            if self.is_update_good(update) {
                acc + Self::get_middle_val(update)
            } else {
                acc
            }
        })
    }
    pub fn get_corrected_middle_total(&self) -> u16 {
        self.updates.clone().iter().fold(0, |acc, update| {
            if self.is_update_good(update) {
                acc
            } else {
                acc + Self::get_middle_val(&self.sort_update(update))
            }
        })
    }
    fn sort_update(&self, _update: &[u16]) -> Vec<u16> {
        // let applicable_rules: Vec<(u16, u16)> = self
        //     .rules
        //     .clone()
        //     .into_iter()
        //     .filter(|rule| update.contains(&rule.0) && update.contains(&rule.1))
        //     .collect();
        // println!("{:?}", update);
        // println!("{:?}", applicable_rules);
        // let mut sorted_update = update.clone();
        // for (idx, num) in sorted_update.iter().enumerate() {
        //     for rule in applicable_rules.iter().filter(|rule| rule.1 == num) {

        //     }
        // }
        // Start with the rule who's first value isn't in any other rules' second value
        // let mut current_rule = applicable_rules.iter().find(|rule1| {
        //     applicable_rules
        //         .iter()
        //         .any(|rule2| rule2.1 == rule1.0)
        // });
        // let mut sorted_vec = vec![];
        // while let Some(current_rule_unwrapped) = current_rule {
        //     sorted_vec.push(current_rule_unwrapped.0);
        //     current_rule = applicable_rules
        //         .iter()
        //         .find(|rule| rule.0 == current_rule_unwrapped.1);
        //     if current_rule.is_none() {
        //         sorted_vec.push(current_rule_unwrapped.1);
        //     }
        // }
        // sorted_update
        unimplemented!();
    }
    fn get_middle_val(update: &[u16]) -> u16 {
        *update.split_at(update.len() / 2).1.first().unwrap()
    }
    fn is_update_good(&self, update: &[u16]) -> bool {
        self.rules.iter().all(|rule| {
            let mut found_second = false;
            for num in update.iter() {
                if num == &rule.1 {
                    found_second = true;
                }
                if num == &rule.0 && found_second {
                    return false;
                }
            }
            true
        })
    }
}

#[cfg(test)]
mod parser {
    use crate::day5::Updater;

    use super::parser;

    #[test]
    fn test_parser() {
        assert_eq!(
            parser("./src/day5/test_data").unwrap(),
            Updater {
                rules: vec![
                    (47, 53),
                    (97, 13),
                    (97, 61),
                    (97, 47),
                    (75, 29),
                    (61, 13),
                    (75, 53),
                    (29, 13),
                    (97, 29),
                    (53, 29),
                    (61, 53),
                    (97, 53),
                    (61, 29),
                    (47, 13),
                    (75, 47),
                    (97, 75),
                    (47, 61),
                    (75, 61),
                    (47, 29),
                    (75, 13),
                    (53, 13)
                ],
                updates: vec![
                    vec![75, 47, 61, 53, 29],
                    vec![97, 61, 53, 29, 13],
                    vec![75, 29, 13],
                    vec![75, 97, 47, 61, 53],
                    vec![61, 13, 29],
                    vec![97, 13, 75, 29, 47]
                ],
            }
        )
    }
}

#[cfg(test)]
mod get_valid_middle_total {

    use super::parser;

    #[test]
    fn test_get_valid_middle_total() {
        assert_eq!(
            parser("./src/day5/test_data")
                .unwrap()
                .get_valid_middle_total(),
            143
        )
    }
}

#[cfg(test)]
mod is_update_good {

    use super::parser;

    #[test]
    fn test_is_update_good() {
        let updater = parser("./src/day5/test_data").unwrap();
        assert!(updater.is_update_good(&vec![75, 47, 61, 53, 29]),);
        assert!(updater.is_update_good(&vec![97, 61, 53, 29, 13]),);
        assert!(updater.is_update_good(&vec![75, 29, 13]),);
        assert!(!updater.is_update_good(&vec![75, 97, 47, 61, 53]),);
        assert!(!updater.is_update_good(&vec![61, 13, 29]),);
        assert!(!updater.is_update_good(&vec![97, 13, 75, 29, 47]),);
    }
}
#[cfg(test)]
// mod sort_update {

//     use super::parser;

//     #[test]
//     fn test_sort_update() {
//         let updater = parser("./src/day5/test_data").unwrap();
//         assert_eq!(
//             updater.sort_update(&vec![75, 97, 47, 61, 53]),
//             vec![97, 75, 47, 61, 53]
//         );
//         assert_eq!(updater.sort_update(&vec![61, 13, 29]), vec![61, 29, 13]);
//         assert_eq!(
//             updater.sort_update(&vec![97, 13, 75, 29, 47]),
//             vec![97, 75, 47, 29, 13]
//         );
//     }
// }
#[cfg(test)]
mod get_middle_val {
    use super::Updater;

    #[test]
    fn test_get_middle_val() {
        assert_eq!(Updater::get_middle_val(&vec![75, 47, 61, 53, 29]), 61);
        assert_eq!(Updater::get_middle_val(&vec![97, 61, 53, 29, 13]), 53);
        assert_eq!(Updater::get_middle_val(&vec![75, 29, 13]), 29);
        assert_eq!(Updater::get_middle_val(&vec![75, 97, 47, 61, 53]), 47);
        assert_eq!(Updater::get_middle_val(&vec![61, 13, 29]), 13);
        assert_eq!(Updater::get_middle_val(&vec![97, 13, 75, 29, 47]), 75);
    }
}
