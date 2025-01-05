use std::collections::HashMap;

use aocr2024::{read_file_to_string, string_to_u64};

fn main() {
    let stones = parse_input("./src/bin/day11/input.txt");
    // Part 1
    let result = blink(25, &stones);
    println!("Part 1: {}", result.len());

    // Part 2
    let count = blink_count(75, &stones);
    println!("Part 2: {}", count);
}

fn parse_input(file_path: &str) -> Vec<u64> {
    read_file_to_string(file_path).split_whitespace().map(|string| string_to_u64(string)).collect()
}

fn has_even_digits(n: u64) -> bool {
    let num_digits = (n as f64).log10().floor() as u32 + 1;
    num_digits % 2 == 0
}

fn split_in_half(n: u64) -> (u64, u64) {
    let num_digits = (n as f64).log10().floor() as u32 + 1;
    let half = 10u64.pow(num_digits / 2);
    (n / half, n % half)
}

fn apply_rule(stone: u64) -> Vec<u64> {
    match stone {
        0 => vec![1],
        stone if has_even_digits(stone) => {
            let (left, right) = split_in_half(stone);
            vec![left, right]
        }
        _ => vec![stone * 2024]
    }
}

fn apply_rules(stones: &Vec<u64>) -> Vec<u64> {
    stones.iter()
        .flat_map(|stone| apply_rule(*stone))
        .collect()
}

fn blink(times: usize, stones: &Vec<u64>) -> Vec<u64> {
    let mut stones = stones.clone();
    for _ in 0..times {
        stones = apply_rules(&stones);
    }
    stones
}

fn blink_count(times: usize, stones: &[u64]) -> usize {
    let mut result_map: HashMap<(u64, usize), usize> = HashMap::new();

    fn recurse(stone: u64, iterations: usize, result_map: &mut HashMap<(u64, usize), usize>) -> usize {
        if iterations == 0 {
            return 1;
        }
        if let Some(cached_result) = result_map.get(&(stone, iterations)) {
            return *cached_result;
        }
        let new_stones = apply_rule(stone);
        let result = new_stones.iter()
            .map(|new_stone| recurse(*new_stone, iterations - 1, result_map))
            .sum::<usize>();
        result_map.insert((stone, iterations), result);
        result
    }

    stones.iter().map(|stone| recurse(*stone, times, &mut result_map)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let file_path = "./src/bin/day11/sample_input.txt";
        let expected = vec![125, 17];
        let result = parse_input(file_path);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_has_even_digits() {
        assert!(!has_even_digits(1));
        assert!(has_even_digits(10));
        assert!(!has_even_digits(100));
        assert!(has_even_digits(1000));
        assert!(!has_even_digits(10_000));
        assert!(has_even_digits(100_000));
        assert!(!has_even_digits(1_000_000));
        assert!(has_even_digits(10_000_000));
        assert!(!has_even_digits(100_000_000));
    }

    #[test]
    fn test_split_in_half() {
        assert_eq!(split_in_half(1234), (12, 34));
        assert_eq!(split_in_half(567890), (567, 890));
    }

    #[test]
    fn test_apply_rule() {
        assert_eq!(apply_rule(0), vec![1]);
        assert_eq!(apply_rule(10), vec![1, 0]);
        assert_eq!(apply_rule(1234), vec![12, 34]);
        assert_eq!(apply_rule(5), vec![5 * 2024]);
    }

    #[test]
    fn test_apply_rules() {
        assert_eq!(apply_rules(&vec![0, 10, 1234, 5]), vec![1, 1, 0, 12, 34, 5 * 2024]);
    }

    #[test]
    fn test_blink() {
        let stones = vec![10, 1234, 5];
        let result = blink(1, &stones);
        assert_eq!(result, vec![1, 0, 12, 34, 5 * 2024]);

        let result = blink(2, &stones);
        assert_eq!(result, vec![1 * 2024, 1, 1, 2, 3, 4, 5 * 2024 * 2024]);
    }

    #[test]
    fn test_blink_count() {
        let stones = vec![10, 1234, 5];
        let result = blink_count(1, &stones);
        assert_eq!(result, 5);

        let result = blink_count(2, &stones);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part_1() {
        let stones = parse_input("./src/bin/day11/sample_input.txt");
        assert_eq!(blink(6, &stones).len(), 22);
        assert_eq!(blink(25, &stones).len(), 55312);
    }
}