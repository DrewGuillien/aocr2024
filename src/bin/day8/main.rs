use std::collections::{HashMap, HashSet};
use antenna_map::{AntennaMap, Map, Position};
use aocr2024::read_file_to_string;

mod antenna_map;

fn main() {
    let map = parse_input("./src/bin/day8/input.txt");
    let antinode_count = map.count_all_antinodes_in_bounds();
    println!("Antinode count: {}", antinode_count);
    let resonant_antinode_count = map.count_all_antinodes_with_resonant_harmonics_in_bounds();
    println!("Resonant antinode count: {}", resonant_antinode_count);
}

fn parse_input(file_path: &str) -> Map {
    let mut antenna_map: AntennaMap = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    read_file_to_string(file_path).chars()
        .for_each(|char| {
            match char {
                '.' => { x += 1 }
                '\n' => {
                    x = 0;
                    y += 1;
                }
                char => {
                    match antenna_map.get_mut(&char) {
                        Some(set) => {
                            set.insert(Position { x, y });
                        }
                        None => {
                            let mut set = HashSet::<Position>::new();
                            set.insert(Position { x, y });
                            antenna_map.insert(char, set);
                        }
                    }
                    x += 1;
                }
            }
        });
    Map {
        width: x,
        height: y + 1,
        antenna_map
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    #[test]
    fn test_sample_input() {
        let map = parse_input("./src/bin/day8/sample_input.txt");
        assert_eq!(map.count_all_antinodes_in_bounds(), 14);
    }

    #[test]
    fn test_sample_input_resonant_harmonics() {
        let map = parse_input("./src/bin/day8/sample_input.txt");
        assert_eq!(map.count_all_antinodes_with_resonant_harmonics_in_bounds(), 34);
    }
}