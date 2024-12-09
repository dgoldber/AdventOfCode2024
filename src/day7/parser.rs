use crate::day7::equation::Equation;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn parser(file_name: &str) -> io::Result<Vec<Equation>> {
    let root = "./src/day7";
    let file = File::open(format!("{root}/{file_name}"))?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let mut split = line.split(':');
            let result = split.next().unwrap().parse().unwrap();
            let operands = split
                .next()
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|operand_string| operand_string.parse().unwrap())
                .collect();
            Equation { result, operands }
        })
        .collect::<Vec<_>>())
}

#[cfg(test)]
mod test_parser {

    use super::parser;
    use crate::day7::equation::Equation;

    #[test]
    fn test_parser() {
        let equations = parser("test_data").unwrap();
        assert_eq!(
            *equations.get(0).unwrap(),
            Equation {
                result: 190,
                operands: vec![10, 19]
            }
        );
        assert_eq!(
            *equations.get(1).unwrap(),
            Equation {
                result: 3267,
                operands: vec![81, 40, 27]
            }
        );
        assert_eq!(
            *equations.get(2).unwrap(),
            Equation {
                result: 83,
                operands: vec![17, 5]
            }
        );
        assert_eq!(
            *equations.get(3).unwrap(),
            Equation {
                result: 156,
                operands: vec![15, 6]
            }
        );
        assert_eq!(
            *equations.get(4).unwrap(),
            Equation {
                result: 7290,
                operands: vec![6, 8, 6, 15]
            }
        );
        assert_eq!(
            *equations.get(5).unwrap(),
            Equation {
                result: 161011,
                operands: vec![16, 10, 13]
            }
        );
        assert_eq!(
            *equations.get(6).unwrap(),
            Equation {
                result: 192,
                operands: vec![17, 8, 14]
            }
        );
        assert_eq!(
            *equations.get(7).unwrap(),
            Equation {
                result: 21037,
                operands: vec![9, 7, 18, 13]
            }
        );
        assert_eq!(
            *equations.get(8).unwrap(),
            Equation {
                result: 292,
                operands: vec![11, 6, 16, 20]
            }
        );
    }
}
