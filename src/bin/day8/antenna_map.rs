use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub(crate) type AntennaMap = HashMap<char, HashSet<Position>>;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub(crate) struct Position {
    pub(crate) x: i32,
    pub(crate) y: i32
}

pub struct Map {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) antenna_map: AntennaMap
}

impl Map {
    pub(crate) fn count_all_antinodes_in_bounds(&self) -> usize {
        self.antenna_map.iter()
            .flat_map(|(_frequency, positions)| {
                positions.iter()
                    .combinations(2)
                    .flat_map(|pair| {
                        Self::get_antinodes(pair[0], pair[1])
                    })
            })
            .filter(|antinode| self.in_bounds(antinode))
            .unique()
            .count()
    }

    pub(crate) fn count_all_antinodes_with_resonant_harmonics_in_bounds(&self) -> usize {
        self.antenna_map.iter()
            .flat_map(|(_frequency, positions)| {
                positions.iter()
                    .combinations(2)
                    .flat_map(|pair| {
                        self.get_antinodes_with_resonant_harmonics(pair[0], pair[1])
                    })
            })
            .filter(|antinode| self.in_bounds(antinode))
            .unique()
            .count()
    }

    fn get_antinodes(antenna1: &Position, antenna2: &Position) -> [Position; 2] {
        let delta_x = antenna1.x - antenna2.x;
        let delta_y = antenna1.y - antenna2.y;
        [
            Position { x: antenna1.x + delta_x, y: antenna1.y + delta_y },
            Position { x: antenna2.x - delta_x, y: antenna2.y - delta_y },
        ]
    }

    fn get_antinodes_with_resonant_harmonics(&self, antenna1: &Position, antenna2: &Position) -> Vec<Position> {
        let delta_x = antenna1.x - antenna2.x;
        let delta_y = antenna1.y - antenna2.y;
        let mut antinodes = Vec::new();
        let mut position1;
        let mut position2;
        let mut multiplier = 0;
        loop {
            position1 = Position { x: antenna1.x + delta_x * multiplier, y: antenna1.y + delta_y * multiplier };
            position2 = Position { x: antenna2.x - delta_x * multiplier, y: antenna2.y - delta_y * multiplier };
            if !self.in_bounds(&position1) && !self.in_bounds(&position2) {
                break;
            }
            if self.in_bounds(&position1) {
                antinodes.push(position1);
            }
            if self.in_bounds(&position2) {
                antinodes.push(position2);
            }
            multiplier += 1;
        }
        antinodes
    }
    
    fn in_bounds(&self, position: &Position) -> bool {
        (0..self.width).contains(&position.x) && (0..self.height).contains(&position.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_antinodes() {
        let antenna1 = Position { x: 1, y: 1 };
        let antenna2 = Position { x: 3, y: 3 };
        let antinodes = Map::get_antinodes(&antenna1, &antenna2);
        assert_eq!(antinodes[0], Position { x: -1, y: -1 });
        assert_eq!(antinodes[1], Position { x: 5, y: 5 });
    }

    #[test]
    fn test_in_bounds() {
        let map = Map {
            width: 10,
            height: 10,
            antenna_map: HashMap::new(),
        };
        assert!(map.in_bounds(&Position { x: 5, y: 5 }));
        assert!(!map.in_bounds(&Position { x: 10, y: 10 }));
        assert!(!map.in_bounds(&Position { x: -1, y: -1 }));
    }

    #[test]
    fn test_count_all_antinodes_in_bounds() {
        let mut antenna_map = AntennaMap::new();
        antenna_map.insert(
            'A',
            vec![
                Position { x: 3, y: 3 },
                Position { x: 5, y: 5 },
            ]
            .into_iter()
            .collect(),
        );
        let map = Map {
            width: 10,
            height: 10,
            antenna_map,
        };
        assert_eq!(map.count_all_antinodes_in_bounds(), 2);
    }

    #[test]
    fn test_count_all_antinodes_out_of_bounds() {
        let mut antenna_map = AntennaMap::new();
        antenna_map.insert(
            'A',
            vec![
                Position { x: 1, y: 1 },
                Position { x: 9, y: 9 },
            ]
            .into_iter()
            .collect(),
        );
        let map = Map {
            width: 10,
            height: 10,
            antenna_map,
        };
        assert_eq!(map.count_all_antinodes_in_bounds(), 0);
    }
}