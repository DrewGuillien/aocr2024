use std::fs::read_to_string;

pub fn read_file_to_string(file_path: &str) -> String {
    read_to_string(file_path).expect(&format!("Error reading from file path: {}", file_path))
}

pub fn string_to_i32(string: &str) -> i32 {
    string.parse::<i32>().expect(&format!("Invalid input. {} is not a number.", string))
}

pub fn string_to_u64(string: &str) -> u64 {
    string.parse::<u64>().expect(&format!("Invalid input. {} is not a number.", string))
}