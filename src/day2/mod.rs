use std::fs::File;
use std::io::{prelude::*, BufReader};

static MAX_SAFE_DIFFERENCE: u8 = 3;
static MIN_SAFE_DIFFERENCE: u8 = 1;

pub fn day2() {
    let file = File::open("./src/day2/input.txt").expect("File does not exist");
    let reader = BufReader::new(file);

    let data = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("Could not parse file");
    let safe_report_count = data
        .iter()
        .filter(|report| is_safe_report(report, 1))
        .count();
    print!("Safe reports: {}", safe_report_count);
}

pub fn is_safe_report(report: &[u32], error_tolerance: u8) -> bool {
    enum Direction {
        Ascending,
        Descending,
    }
    let mut last_num = None;
    let mut direction: Option<Direction> = None;
    let mut safe = true;
    for num in report.iter() {
        if let Some(last) = last_num {
            let diff = num.abs_diff(last);
            if diff < MIN_SAFE_DIFFERENCE.into() || diff > MAX_SAFE_DIFFERENCE.into() {
                safe = false;
            }
            let ascending = num.gt(&last);
            match (&direction, ascending) {
                (Some(Direction::Ascending), false) => {
                    safe = false;
                }
                (Some(Direction::Descending), true) => {
                    safe = false;
                }
                (None, _) => {
                    direction = if ascending {
                        Some(Direction::Ascending)
                    } else {
                        Some(Direction::Descending)
                    }
                }
                _ => {}
            }
        }
        last_num = Some(*num);
    }
    if safe {
        true
    } else if error_tolerance > 0 {
        for i in 0..report.len() {
            let mut skip_vec = report.to_owned();
            skip_vec.remove(i);
            if is_safe_report(&skip_vec, error_tolerance - 1) {
                return true;
            }
        }
        false
    } else {
        false
    }
}

#[cfg(test)]
mod no_tolerance {
    use super::is_safe_report;

    #[test]
    fn test_safe_increasing() {
        assert!(is_safe_report(&vec![7, 6, 4, 2, 1], 0));
    }
    #[test]
    fn test_safe_decreasing() {
        assert!(is_safe_report(&vec![1, 3, 6, 7, 9], 0));
    }
    #[test]
    fn test_unsafe_big_increase() {
        assert!(!is_safe_report(&vec![1, 2, 7, 8, 9], 0));
    }
    #[test]
    fn test_unsafe_big_decrease() {
        assert!(!is_safe_report(&vec![9, 7, 6, 2, 1], 0));
    }
    #[test]
    fn test_unsafe_no_change() {
        assert!(!is_safe_report(&vec![8, 6, 4, 4, 1], 0));
    }
    #[test]
    fn test_unsafe_unexpected_decrease() {
        assert!(!is_safe_report(&vec![1, 3, 2, 4, 5], 0));
    }
}

#[cfg(test)]
mod one_tolerance {
    use super::is_safe_report;

    #[test]
    fn test_safe_increasing() {
        assert!(is_safe_report(&vec![7, 6, 4, 2, 1], 1));
    }
    #[test]
    fn test_safe_decreasing() {
        assert!(is_safe_report(&vec![1, 3, 6, 7, 9], 1));
    }
    #[test]
    fn test_unsafe_big_increase() {
        assert!(!is_safe_report(&vec![1, 2, 7, 8, 9], 1));
    }
    #[test]
    fn test_unsafe_big_decrease() {
        assert!(!is_safe_report(&vec![9, 7, 6, 2, 1], 1));
    }
    #[test]
    fn test_safe_no_change() {
        assert!(is_safe_report(&vec![8, 6, 4, 4, 1], 1));
    }
    #[test]
    fn test_safe_unexpected_decrease() {
        assert!(is_safe_report(&vec![1, 3, 2, 4, 5], 1));
    }
    #[test]
    fn test_safe_remove_first() {
        assert!(is_safe_report(&vec![2, 8, 9, 10, 11], 1));
    }
}
