use aocr2024::string_to_u64;

#[derive(PartialEq, Eq, Debug)]
pub struct CalibrationEquation {
    pub test_value: u64,
    operands: Vec<u64>
}

impl CalibrationEquation {
    pub fn equals_target_with_operators(&self, operators: &[Operator]) -> bool {
        fn recurse(value: &u64, target: &u64, operands: &[u64], operator: &Operator, operators: &[Operator]) -> bool {
            match operands.split_first() {
                Some((first, rest)) => {
                    let new_value = operator.apply(*value, *first);
                    if rest.is_empty() {
                        return new_value == *target;
                    }
                    if new_value > *target {
                        return false;
                    }
                    operators.iter()
                        .any(|operator| recurse(&new_value, target, rest, operator, operators))
                }
                None => false
            }
        }

        match self.operands.split_first() {
            Some((first, rest)) => {
                operators.iter()
                    .any(|operator| recurse(first, &self.test_value, rest, operator, operators))
            }
            None => false
        }
    }
}

impl From<&str> for CalibrationEquation {
    fn from(value: &str) -> Self {
        let (test_value, operands) = value.split_once(':').expect("Invalid input. Missing ':'");
        Self {
            test_value: string_to_u64(&test_value),
            operands: operands.split_whitespace().map(|operand| string_to_u64(operand)).collect()
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Operator {
    Plus,
    Multiply,
    Concatenate
}

impl Operator {
    pub fn apply(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operator::Plus => lhs + rhs,
            Operator::Multiply => lhs * rhs,
            Operator::Concatenate => {
                let string = format!("{}{}", lhs, rhs);
                string_to_u64(&string)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use aocr2024::read_file_to_string;
    use super::*;

    #[test]
    fn test_sample_input_with_plus_and_multiply() {
        let equations: Vec<CalibrationEquation> = read_file_to_string("./src/bin/day7/sample_input.txt")
            .split("\n")
            .map(|line| line.into())
            .collect();
        let calibration_result: u64 = equations.iter()
            .filter(|equation| equation.equals_target_with_operators(&[Operator::Plus, Operator::Multiply]))
            .map(|equation| equation.test_value)
            .sum();
        assert_eq!(calibration_result, 3749);
    }

    #[test]
    fn test_sample_input_with_plus_and_multiply_and_concatonate() {
        let equations: Vec<CalibrationEquation> = read_file_to_string("./src/bin/day07/sample_input.txt")
            .split("\n")
            .map(|line| line.into())
            .collect();        
        let calibration_result: u64 = equations.iter()
            .filter(|equation| equation.equals_target_with_operators(&[Operator::Plus, Operator::Multiply, Operator::Concatenate]))
            .map(|equation| equation.test_value)
            .sum();
        assert_eq!(calibration_result, 11387);
    }

    #[test]
    fn test_equation_target_equality_with_plus_and_multiply() {
        let operators = [Operator::Plus, Operator::Multiply];
        let equation1 = CalibrationEquation { test_value: 190, operands: vec![10, 19] };
        assert_eq!(equation1.equals_target_with_operators(&operators), true, "Line 1");
        let equation2 = CalibrationEquation { test_value: 3267, operands: vec![81, 40, 27] };
        assert_eq!(equation2.equals_target_with_operators(&operators), true, "Line 2");
        let equation3 = CalibrationEquation { test_value: 83, operands: vec![17, 5] };
        assert_eq!(equation3.equals_target_with_operators(&operators), false, "Line 3");
        let equation4 = CalibrationEquation { test_value: 156, operands: vec![15, 6] };
        assert_eq!(equation4.equals_target_with_operators(&operators), false, "Line 4");
        let equation5 = CalibrationEquation { test_value: 7290, operands: vec![6, 8, 6, 15] };
        assert_eq!(equation5.equals_target_with_operators(&operators), false, "Line 5");
        let equation6 = CalibrationEquation { test_value: 161011, operands: vec![16, 10, 13] };
        assert_eq!(equation6.equals_target_with_operators(&operators), false, "Line 6");
        let equation7 = CalibrationEquation { test_value: 192, operands: vec![17, 8, 14] };
        assert_eq!(equation7.equals_target_with_operators(&operators), false, "Line 7");
        let equation8 = CalibrationEquation { test_value: 21037, operands: vec![9, 7, 18, 13] };
        assert_eq!(equation8.equals_target_with_operators(&operators), false, "Line 8");
        let equation9 = CalibrationEquation { test_value: 292, operands: vec![11, 6, 16, 20] };
        assert_eq!(equation9.equals_target_with_operators(&operators), true, "Line 9");
    }

    #[test]
    fn test_equation_target_equality_with_plus_and_multiply_and_concatonate() {
        let operators = [Operator::Plus, Operator::Multiply, Operator::Concatenate];
        let equation1 = CalibrationEquation { test_value: 190, operands: vec![10, 19] };
        assert_eq!(equation1.equals_target_with_operators(&operators), true, "Line 1");
        let equation2 = CalibrationEquation { test_value: 3267, operands: vec![81, 40, 27] };
        assert_eq!(equation2.equals_target_with_operators(&operators), true, "Line 2");
        let equation3 = CalibrationEquation { test_value: 83, operands: vec![17, 5] };
        assert_eq!(equation3.equals_target_with_operators(&operators), false, "Line 3");
        let equation4 = CalibrationEquation { test_value: 156, operands: vec![15, 6] };
        assert_eq!(equation4.equals_target_with_operators(&operators), true, "Line 4");
        let equation5 = CalibrationEquation { test_value: 7290, operands: vec![6, 8, 6, 15] };
        assert_eq!(equation5.equals_target_with_operators(&operators), true, "Line 5");
        let equation6 = CalibrationEquation { test_value: 161011, operands: vec![16, 10, 13] };
        assert_eq!(equation6.equals_target_with_operators(&operators), false, "Line 6");
        let equation7 = CalibrationEquation { test_value: 192, operands: vec![17, 8, 14] };
        assert_eq!(equation7.equals_target_with_operators(&operators), true, "Line 7");
        let equation8 = CalibrationEquation { test_value: 21037, operands: vec![9, 7, 18, 13] };
        assert_eq!(equation8.equals_target_with_operators(&operators), false, "Line 8");
        let equation9 = CalibrationEquation { test_value: 292, operands: vec![11, 6, 16, 20] };
        assert_eq!(equation9.equals_target_with_operators(&operators), true, "Line 9");
    }

    #[test]
    fn test_equation_from_string() {
        let line1 = "190: 10 19";
        let equation1 = CalibrationEquation { test_value: 190, operands: vec![10, 19] };
        assert_eq!(Into::<CalibrationEquation>::into(line1), equation1);
        let line2 = "3267: 81 40 27";
        let equation2 = CalibrationEquation { test_value: 3267, operands: vec![81, 40, 27] };
        assert_eq!(Into::<CalibrationEquation>::into(line2), equation2);
        let line3 = "83: 17 5";
        let equation3 = CalibrationEquation { test_value: 83, operands: vec![17, 5] };
        assert_eq!(Into::<CalibrationEquation>::into(line3), equation3);
        let line4 = "156: 15 6";
        let equation4 = CalibrationEquation { test_value: 156, operands: vec![15, 6] };
        assert_eq!(Into::<CalibrationEquation>::into(line4), equation4);
        let line5 = "7290: 6 8 6 15";
        let equation5 = CalibrationEquation { test_value: 7290, operands: vec![6, 8, 6, 15] };
        assert_eq!(Into::<CalibrationEquation>::into(line5), equation5);
        let line6 = "161011: 16 10 13";
        let equation6 = CalibrationEquation { test_value: 161011, operands: vec![16, 10, 13] };
        assert_eq!(Into::<CalibrationEquation>::into(line6), equation6);
        let line7 = "192: 17 8 14";
        let equation7 = CalibrationEquation { test_value: 192, operands: vec![17, 8, 14] };
        assert_eq!(Into::<CalibrationEquation>::into(line7), equation7);
        let line8 = "21037: 9 7 18 13";
        let equation8 = CalibrationEquation { test_value: 21037, operands: vec![9, 7, 18, 13] };
        assert_eq!(Into::<CalibrationEquation>::into(line8), equation8);
        let line9 = "292: 11 6 16 20";
        let equation9 = CalibrationEquation { test_value: 292, operands: vec![11, 6, 16, 20] };
        assert_eq!(Into::<CalibrationEquation>::into(line9), equation9);
    }

    #[test]
    fn test_plus_operator() {
        let operator = Operator::Plus;
        assert_eq!(operator.apply(0, 0), 0);
        assert_eq!(operator.apply(1, 1), 2);
        assert_eq!(operator.apply(0, 1), 1);
        assert_eq!(operator.apply(1, 0), 1);
        assert_eq!(operator.apply(2, 2), 4);
        // That should be enough for this
    }

    #[test]
    fn test_multiply_operator() {
        let operator = Operator::Multiply;
        assert_eq!(operator.apply(0, 0), 0);
        assert_eq!(operator.apply(0, 1), 0);
        assert_eq!(operator.apply(0, 100), 0);
        assert_eq!(operator.apply(1, 1), 1);
        assert_eq!(operator.apply(1, 2), 2);
        assert_eq!(operator.apply(2, 2), 4);
    }
}