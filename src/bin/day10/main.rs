use std::vec;

use aocr2024::read_file_to_string;
use itertools::Itertools;


fn main() {
    let topographical_map = parse_input("src/bin/day10/input.txt");
    // Part 1
    let result = score_trailheads(&topographical_map);
    println!("Result: {}", result);

    // Part 2
    let result = rate_trailheads(&topographical_map);
    println!("Result: {}", result);
}

fn parse_input(file_path: &str) -> Vec<Vec<u32>> {
    read_file_to_string(file_path)
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn score_trailheads(topographical_map: &Vec<Vec<u32>>) -> usize {
    fn recurse(x: usize, y: usize, topographical_map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
        let height = topographical_map.len();
        let width = topographical_map[0].len();
        let current = topographical_map[y][x];
        if current == 9 {
            return vec![(x, y)];
        }
        [
            (x as isize, y as isize - 1),
            (x as isize + 1, y as isize),
            (x as isize, y as isize + 1),
            (x as isize - 1, y as isize),
        ].iter()
            .filter_map(|&(x, y)| {
            if x >= 0 && y >= 0 && x < width as isize && y < height as isize {
                Some((x as usize, y as usize))
            } else {
                None
            }
            })
            .filter(|(x, y)| {
                topographical_map[*y][*x] == current + 1
            }).map(|(x, y)| {
                recurse(x, y, topographical_map)
            }).flatten().collect()
    }
    topographical_map.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().map(|(x, elevation)| {
            if *elevation != 0 {
                return 0;
            }
            recurse(x, y, topographical_map).iter().unique().count()
        }).sum::<usize>()
    }).sum()
}

fn rate_trailheads(topographical_map: &Vec<Vec<u32>>) -> usize {
    fn recurse(x: usize, y: usize, topographical_map: &Vec<Vec<u32>>) -> usize {
        let height = topographical_map.len();
        let width = topographical_map[0].len();
        let current = topographical_map[y][x];
        if current == 9 {
            return 1;
        }
        [
            (x as isize, y as isize - 1),
            (x as isize + 1, y as isize),
            (x as isize, y as isize + 1),
            (x as isize - 1, y as isize),
        ].iter()
            .filter_map(|&(x, y)| {
            if x >= 0 && y >= 0 && x < width as isize && y < height as isize {
                Some((x as usize, y as usize))
            } else {
                None
            }
            })
            .filter(|(x, y)| {
                topographical_map[*y][*x] == current + 1
            }).map(|(x, y)| {
                recurse(x, y, topographical_map)
            }).sum()
    }
    topographical_map.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().map(|(x, elevation)| {
            if *elevation != 0 {
                return 0;
            }
            recurse(x, y, topographical_map)
        }).sum::<usize>()
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let expected = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ];
        assert_eq!(parse_input("src/bin/day10/sample_input.txt"), expected);
    }

    #[test]
    fn test_score_trailheads() {
        let topographical_map = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ];
        assert_eq!(score_trailheads(&topographical_map), 36);
    }

    #[test]
    fn test_rate_trailheads() {
        let topographical_map = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ];
        assert_eq!(rate_trailheads(&topographical_map), 81);
    }
}