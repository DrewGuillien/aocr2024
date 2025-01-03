use aocr2024::read_file_to_string;
use equation::{CalibrationEquation, Operator};

mod equation;

fn main() {
    let equations = parse_input("./src/bin/day07/input.txt");
    // Part 1
    let calibration_result: u64 = equations.iter()
        .filter(|equation| equation.equals_target_with_operators(&[Operator::Plus, Operator::Multiply]))
        .map(|equation| equation.test_value)
        .sum();
    println!("Calibration result: {}", calibration_result);
    // Part 2
    let altered_calibration_result: u64 = equations.iter()
        .filter(|equation| equation.equals_target_with_operators(&[Operator::Plus, Operator::Multiply, Operator::Concatenate]))
        .map(|equation| equation.test_value)
        .sum();
    println!("Altered calibration result: {}", altered_calibration_result);
}

fn parse_input(file_path: &str) -> Vec<CalibrationEquation> {
    read_file_to_string(file_path)
        .split("\n")
        .map(|line| line.into())
        .collect()
}

#[cfg(test)]
mod tests {
    
}