use regex::Regex;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let instructions = match args.len() {
        2 => {
            parse_input(&args[1])
        }
        _ => {
            parse_input("./src/bin/day3/input.txt")
        }
    };
    // Part 1
    let value = interpret(&instructions);
    println!("{}", value);
    // Part 2
    let new_value = conditional_interpret(&instructions);
    println!("{}", new_value);
}

fn parse_input(file_path: &str) -> String {
    std::fs::read_to_string(file_path).expect(&format!("Error reading file path: {}", file_path))
}

fn interpret(instructions: &str) -> i32 {
    Regex::new(r"mul\((\d*?),(\d*?)\)")
        .unwrap()
        .captures_iter(instructions)
        .map(|captures| {
            captures
                .extract::<2>()
                .1
                .iter()
                .fold(1, |accumulator, capture| accumulator * capture.parse::<i32>().unwrap())
        }).sum()
}

fn conditional_interpret(instructions: &str) -> i32 {
    // (?s) flag is required here to set dotall mode so that . includes newlines
    let new_instructions = Regex::new(r"(?s)(don't\(\).*?(?:do\(\)|$))")
        .unwrap()
        .replace_all(instructions, "");
    interpret(&new_instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let expected = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(parse_input("./src/bin/day3/sample_input.txt"), expected);
    }

    #[test]
    fn test_interpret() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(interpret(input), 161);
    }

    #[test]
    fn test_conditional_interpret() {
        let input = "xdo()mul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))don't()mul(1,2)";
        assert_eq!(conditional_interpret(input), 48);
    }

    #[test]
    fn test_conditional_interpret_with_dangling_dont() {
        let input = "mul(1,2)mul(2,2)don't()mul(100,100)";
        assert_eq!(conditional_interpret(input), 6);
    }

    #[test]
    fn test_conditional_interpret_consecutive_donts() {
        let input = "mul(1,2)don't()don't()don't()mul(1,5)don't()mul(1,100)do()";
        assert_eq!(conditional_interpret(input), 2);
    }

    #[test]
    fn test_conditional_interpret_with_newlines() {
        let input = "mul(1,2)don't()\nmul(1,5)don't()mul(1,100)do()";
        assert_eq!(conditional_interpret(input), 2);
    }
}