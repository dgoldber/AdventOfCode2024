#[derive(PartialEq, Eq, Debug)]
pub struct Equation {
    pub result: u64,
    pub operands: Vec<u64>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Equation {
    fn find_valid_operators_for_2(val1: u64, val2: u64, result: u64) -> Option<Operator> {
        // println!("{val1} || {val2} = {}", format!("{val1}{val2}").parse::<u64>().unwrap());
        if val1 + val2 == result {
            Some(Operator::Add)
        } else if val1 * val2 == result {
            Some(Operator::Multiply)
        } else if format!("{val1}{val2}").parse::<u64>().unwrap() == result {
            // println!("{}", format!("{val1}{val2}").parse::<u64>().unwrap());
            Some(Operator::Concatenate)
        } else {
            None
        }
    }

    pub fn find_valid_operators(&self) -> Option<Vec<Operator>> {
        Self::find_valid_operators_inner(self.operands.clone(), self.result).map(|mut operators| {
            // Since the operators were figured out depth first, we need to reverse the list
            operators.reverse();
            operators
        })
    }

    fn find_valid_operators_inner(operands: Vec<u64>, result: u64) -> Option<Vec<Operator>> {
        // Base case where only 2 operands are left
        if operands.len() == 2 {
            return Self::find_valid_operators_for_2(
                *operands.first().unwrap(),
                *operands.get(1).unwrap(),
                result,
            )
            .map(|operator| vec![operator]);
        }

        // Check of the next operator is Add
        let mut operands_clone_for_add = operands.clone();
        let (first_operand_for_add, rest_of_the_operands_for_add) =
            operands_clone_for_add.split_first_mut().unwrap();
        let next_operand_for_add = rest_of_the_operands_for_add.get_mut(0).unwrap();
        *next_operand_for_add += *first_operand_for_add;
        if let Some(mut add_result) =
            Self::find_valid_operators_inner(rest_of_the_operands_for_add.to_vec(), result)
        {
            add_result.push(Operator::Add);
            return Some(add_result);
        }

        // Check if the next operator is Multiply
        let mut operands_clone_for_mult = operands.clone();
        let (first_operand_for_mult, rest_of_the_operands_for_mult) =
            operands_clone_for_mult.split_first_mut().unwrap();
        let next_operand_for_mult = rest_of_the_operands_for_mult.get_mut(0).unwrap();
        *next_operand_for_mult *= *first_operand_for_mult;
        if let Some(mut mult_result) =
            Self::find_valid_operators_inner(rest_of_the_operands_for_mult.to_vec(), result)
        {
            mult_result.push(Operator::Multiply);
            return Some(mult_result);
        }

        // Check if the next operator is Concatenate
        let mut operands_clone_for_concat = operands.clone();
        let (first_operand_for_concat, rest_of_the_operands_for_concat) =
            operands_clone_for_concat.split_first_mut().unwrap();
        let next_operand_for_concat = rest_of_the_operands_for_concat.get_mut(0).unwrap();
        *next_operand_for_concat =
            format!("{}{}", *first_operand_for_concat, *next_operand_for_concat)
                .parse::<u64>()
                .unwrap();
        // println!("{:?}", rest_of_the_operands_for_concat);
        if let Some(mut concat_result) =
            Self::find_valid_operators_inner(rest_of_the_operands_for_concat.to_vec(), result)
        {
            concat_result.push(Operator::Concatenate);
            return Some(concat_result);
        }

        None
    }

    pub fn get_total_calibration_results(equations: Vec<Self>) -> u64 {
        equations.iter().fold(0, |acc, equation| {
            if equation.find_valid_operators().is_some() {
                acc + equation.result
            } else {
                acc
            }
        })
    }
}

#[cfg(test)]
mod test_equation {

    use super::*;
    use crate::day7::parser::parser;

    #[test]
    fn test_find_valid_operators() {
        let equations = parser("test_data").unwrap();
        assert_eq!(
            equations.get(0).unwrap().find_valid_operators(),
            Some(vec![Operator::Multiply])
        );
        assert_eq!(
            equations.get(1).unwrap().find_valid_operators(),
            Some(vec![Operator::Add, Operator::Multiply])
        );
        assert_eq!(equations.get(2).unwrap().find_valid_operators(), None);
        assert_eq!(
            equations.get(3).unwrap().find_valid_operators(),
            Some(vec![Operator::Concatenate])
        );
        assert_eq!(
            equations.get(4).unwrap().find_valid_operators(),
            Some(vec![
                Operator::Multiply,
                Operator::Concatenate,
                Operator::Multiply
            ])
        );
        assert_eq!(equations.get(5).unwrap().find_valid_operators(), None);
        assert_eq!(
            equations.get(6).unwrap().find_valid_operators(),
            Some(vec![Operator::Concatenate, Operator::Add])
        );
        assert_eq!(equations.get(7).unwrap().find_valid_operators(), None);
        assert_eq!(
            equations.get(8).unwrap().find_valid_operators(),
            Some(vec![Operator::Add, Operator::Multiply, Operator::Add])
        );
    }

    #[test]
    fn test_get_total_calibration_results() {
        let equations = parser("test_data").unwrap();
        // assert_eq!(Equation::get_total_calibration_results(equations), 3749);
        assert_eq!(Equation::get_total_calibration_results(equations), 11387);
    }
}
