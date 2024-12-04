use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn day1() -> io::Result<()> {
    let file = File::open("./src/day1/input.txt")?;
    let reader = BufReader::new(file);
    let mut left_list: Vec<u32> = vec![];
    let mut right_list: Vec<u32> = vec![];

    for line in reader.lines().map_while(Result::ok) {
        let mut split = line.split_whitespace();
        left_list.push(split.next().unwrap().parse().unwrap());
        right_list.push(split.next().unwrap().parse().unwrap());
    }

    let total_distance = calc_total_distance(&left_list, &right_list);
    let similarity_score = calc_similarity_score(&left_list, &right_list);

    println!("total distance: {}", total_distance);
    println!("similarity score: {}", similarity_score);

    Ok(())
}

fn calc_total_distance(left_list: &[u32], right_list: &[u32]) -> u32 {
    let mut left_list = left_list.to_owned();
    let mut right_list = right_list.to_owned();
    left_list.sort();
    right_list.sort();
    // Doing this as a for loop would be slightly more readable, but this is more fun
    left_list.iter().enumerate().fold(0, |acc, (i, left_val)| {
        acc + left_val.abs_diff(*right_list.get(i).unwrap())
    })
}

fn calc_similarity_score(left_list: &[u32], right_list: &[u32]) -> u32 {
    left_list.iter().fold(0, |acc, left_val| {
        acc + left_val
            * u32::try_from(right_list.iter().filter(|val| left_val == *val).count()).unwrap()
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(
            calc_total_distance(&vec![3, 4, 2, 1, 3, 3], &vec![4, 3, 5, 3, 9, 3]),
            11
        );
    }

    #[test]
    fn test_similarity_scores_empty() {
        let left_list = vec![];
        let right_list = vec![];
        assert_eq!(calc_similarity_score(&left_list, &right_list), 0);
    }
    #[test]
    fn test_similarity_scores() {
        assert_eq!(
            calc_similarity_score(&vec![3, 4, 2, 1, 3, 3], &vec![4, 3, 5, 3, 9, 3]),
            31
        );
    }
}
