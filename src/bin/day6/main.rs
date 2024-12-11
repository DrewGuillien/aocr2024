use std::fs::read_to_string;
use area::Area;

mod area;

fn main() {
    // Most of the code is in `./src/bin/day6/area.rs`
    let mut area = parse_input("./src/bin/day6/input.txt");
    let unique_position_count = area.how_many_distinct_positions();
    // Part 1
    println!("Unique positions: {}", unique_position_count);
    let unique_obstacles = area.how_many_unique_new_obstacles_cause_a_loop();
    println!("Unique obstacle count: {}", unique_obstacles);
}

fn parse_input(file_path: &str) -> Area {
    read_to_string(file_path)
        .expect(&format!("Invalid file path: {}", file_path))
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        let input = read_to_string("./src/bin/day6/sample_input.txt")
            .unwrap();
        let mut area: Area = input.into();
        assert_eq!(area.how_many_distinct_positions(), 41);
    }

    #[test]
    fn test_sample_input_part_2() {
        let input = read_to_string("./src/bin/day6/sample_input.txt")
            .unwrap();
        let mut area: Area = input.into();
        assert_eq!(area.how_many_unique_new_obstacles_cause_a_loop(), 6);
    }
}