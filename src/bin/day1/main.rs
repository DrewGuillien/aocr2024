
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (left_list, right_list) = match args.len() {
        2 => {
            parse_input(&args[1])
        }
        _ => {
            parse_input("./src/bin/day1/input.txt")
        }
    };
    let sum = sum_of_distances(left_list, right_list);
    println!("{}", sum);
}

fn parse_input(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let input_string = std::fs::read_to_string(file_path)
        .expect(&format!("Error reading file path {}", file_path));
    let mut left_list = Vec::<i32>::new();
    let mut right_list = Vec::<i32>::new();
    input_string.split("\n").for_each(|line| {
        let nums: Vec<i32> = line.split_whitespace().map(|item| {
            item.parse::<i32>().expect(&format!("Invalid input. {} is not a number.", item))
        }).collect();
        left_list.push(nums[0]);
        right_list.push(nums[1]);
    });
    left_list.sort();
    right_list.sort();
    (left_list, right_list)
}

fn sum_of_distances(left_list: Vec<i32>, right_list: Vec<i32>) -> i32 {
    left_list.into_iter().zip(right_list).map(|(left, right)| (left - right).abs()).sum()
}

#[cfg(test)]
mod tests {
    use std::vec;
    use super::*;

    #[test]
    fn test_sum_of_distances() {
        let left_list = vec![1, 2, 3, 3, 3, 4];
        let right_list = vec![3, 3, 3, 4, 5, 9];
        assert_eq!(sum_of_distances(left_list, right_list), 11);
    }

    #[test]
    fn test_parse_file() {
        let file_path = "./src/bin/day1/sample_input.txt";
        let left_list = vec![1, 2, 3, 3, 3, 4];
        let right_list = vec![3, 3, 3, 4, 5, 9];
        assert_eq!(parse_input(file_path), (left_list, right_list));
    }
}