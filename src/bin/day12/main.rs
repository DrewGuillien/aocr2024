use std::{collections::HashSet, usize::MAX, vec};

use aocr2024::read_file_to_string;

fn main() {
    let input = parse_input("./src/bin/day12/input.txt");
    let regions = get_regions(&input);
    // Part 1
    let cost = regions.iter().map(|region| get_cost_to_fence(region)).sum::<usize>();
    println!("The total cost to fence the regions is: {}", cost);

    // Part 2
    let discounted_cost = regions.iter().map(|region| get_discounted_cost_to_fence(region)).sum::<usize>();
    println!("The total discounted cost to fence the regions is: {}", discounted_cost);
}

fn parse_input(file_path: &str) -> Vec<Vec<char>> {
    let input = read_file_to_string(file_path);
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_regions(input: &Vec<Vec<char>>) -> Vec<HashSet<(usize, usize)>> {
    let mut regions = Vec::new();
    let mut found_regions = HashSet::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if !found_regions.contains(&(x, y)) {
                let mut region = HashSet::new();
                let mut searched = HashSet::new();
                let mut stack = Vec::new();
                let region_id = input[y][x];
                stack.push((x, y));
                while let Some((x, y)) = stack.pop() {
                    if x == MAX || y == MAX || y >= input.len() || x >= input[y].len() || searched.contains(&(x, y)) {
                        continue;
                    }
                    if !found_regions.contains(&(x, y)) && input[y][x] == region_id {
                        searched.insert((x, y));
                        region.insert((x, y));
                        stack.push((x + 1, y));
                        stack.push((x.wrapping_sub(1), y));
                        stack.push((x, y + 1));
                        stack.push((x, y.wrapping_sub(1)));
                    }
                }
                if region.len() > 0 {
                    found_regions.extend(region.clone());
                    regions.push(region);
                }
            }
        }
    }
    regions
}

fn get_cost_to_fence(region: &HashSet<(usize, usize)>) -> usize {
    let area = region.len();
    let perimiter: usize = region.iter().map(|(x, y)| {
        vec![
            (x + 1, *y),
            (x.wrapping_sub(1), *y),
            (*x, y + 1),
            (*x, y.wrapping_sub(1)),
        ].iter().filter(|(x, y)| !region.contains(&(*x, *y))).count()
    }).sum();
    area * perimiter
}

fn get_discounted_cost_to_fence(region: &HashSet<(usize, usize)>) -> usize {
    let area = region.len();
    // Corner detection. A region will have the same amount of sides as the amount of corners.
    // Map each point to the amount of corners it creates.
    let sides = region.iter().map(|(x, y)| {
        vec![
            vec![ (x.wrapping_sub(1), *y), (*x, y.wrapping_sub(1)), (x.wrapping_sub(1), y.wrapping_sub(1)) ],
            vec![ (x.wrapping_sub(1), *y), (*x, y + 1), (x.wrapping_sub(1), y + 1) ],
            vec![ (x + 1, *y), (*x, y.wrapping_sub(1)), (x + 1, y.wrapping_sub(1)) ],
            vec![ (x + 1, *y), (*x, y + 1), (x + 1, y + 1) ],
        ].iter().filter(|corner| {
            (!region.contains(&corner[0]) && !region.contains(&corner[1])) ||
            (region.contains(&corner[0]) && region.contains(&corner[1]) && !region.contains(&corner[2]))
        }).count()
    }).sum::<usize>();

    area * sides
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = parse_input("./src/bin/day12/sample_input.txt");
        let expected = vec![
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F'],
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'C', 'F'],
            vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F'],
            vec!['V', 'V', 'R', 'C', 'C', 'C', 'J', 'F', 'F', 'F'],
            vec!['V', 'V', 'V', 'V', 'C', 'J', 'J', 'C', 'F', 'E'],
            vec!['V', 'V', 'I', 'V', 'C', 'C', 'J', 'J', 'E', 'E'],
            vec!['V', 'V', 'I', 'I', 'I', 'C', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'I', 'I', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'S', 'I', 'J', 'E', 'E', 'E'],
            vec!['M', 'M', 'M', 'I', 'S', 'S', 'J', 'E', 'E', 'E'],
        ];
        assert_eq!(input, expected);
    }

    #[test]
    fn test_get_regions() {
        let input = vec![
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F'],
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'C', 'F'],
            vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F'],
            vec!['V', 'V', 'R', 'C', 'C', 'C', 'J', 'F', 'F', 'F'],
            vec!['V', 'V', 'V', 'V', 'C', 'J', 'J', 'C', 'F', 'E'],
            vec!['V', 'V', 'I', 'V', 'C', 'C', 'J', 'J', 'E', 'E'],
            vec!['V', 'V', 'I', 'I', 'I', 'C', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'I', 'I', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'S', 'I', 'J', 'E', 'E', 'E'],
            vec!['M', 'M', 'M', 'I', 'S', 'S', 'J', 'E', 'E', 'E'],
        ];
        let regions = get_regions(&input);
        let expected = vec![
            // R
            vec![
                (0, 0), (1, 0), (2, 0), (3, 0),
                (0, 1), (1, 1), (2, 1), (3, 1),
                                (2, 2), (3, 2), (4, 2),
                                (2, 3)
            ].into_iter().collect(),
            // I
            vec![
                (4, 0), (5, 0),
                (4, 1), (5, 1),
            ].into_iter().collect(),
            // C
            vec![
                                        (6, 0), (7, 0),
                                        (6, 1), (7, 1), (8, 1),
                                (5, 2), (6, 2),
                (3, 3), (4, 3), (5, 3),
                        (4, 4),
                        (4, 5), (5, 5),
                                (5, 6),
            ].into_iter().collect(),
            // F
            vec![
                                (8, 0), (9, 0),
                                        (9, 1),
                        (7, 2), (8, 2), (9, 2),
                        (7, 3), (8, 3), (9, 3),
                                (8, 4)
            ].into_iter().collect(),
            // V
            vec![
                (0, 2), (1, 2),
                (0, 3), (1, 3),
                (0, 4), (1, 4), (2, 4), (3, 4),
                (0, 5), (1, 5),         (3, 5),
                (0, 6), (1, 6),
            ].into_iter().collect(),
            // J
            vec![
                        (6, 3),
                (5, 4), (6, 4),
                        (6, 5), (7, 5),
                        (6, 6), (7, 6),
                        (6, 7), (7, 7),
                        (6, 8),
                        (6, 9),
            ].into_iter().collect(),
            // C
            vec![
                (7, 4)
            ].into_iter().collect(),
            // E
            vec![
                                (9, 4),
                        (8, 5), (9, 5),
                        (8, 6), (9, 6),
                        (8, 7), (9, 7),
                (7, 8), (8, 8), (9, 8),
                (7, 9), (8, 9), (9, 9),
            ].into_iter().collect(),
            // I
            vec![
                        (2, 5),
                        (2, 6), (3, 6), (4, 6),
                (1, 7), (2, 7), (3, 7), (4, 7), (5, 7),
                (1, 8), (2, 8), (3, 8),         (5, 8),
                                (3, 9)
            ].into_iter().collect(),
            // M
            vec![
                (0, 7),
                (0, 8),
                (0, 9), (1, 9), (2, 9)
            ].into_iter().collect(),
            // S
            vec![
                (4, 8),
                (4, 9), (5, 9),
            ].into_iter().collect(),
        ];
        for (index, region) in regions.iter().enumerate() {
            assert_eq!(*region, expected[index]);
        }
    }

    #[test]
    fn test_get_cost_to_fence() {
        let region_r = vec![
            (0, 0), (1, 0), (2, 0), (3, 0),
            (0, 1), (1, 1), (2, 1), (3, 1),
                            (2, 2), (3, 2), (4, 2),
                            (2, 3)
        ].into_iter().collect();
        let cost_r = get_cost_to_fence(&region_r);
        assert_eq!(cost_r, 216, "Cost of region R is not correct");
        let region_i1 = vec![
            (4, 0), (5, 0),
            (4, 1), (5, 1),
        ].into_iter().collect();
        let cost_i1 = get_cost_to_fence(&region_i1);
        assert_eq!(cost_i1, 32, "Cost of region I (1) is not correct");
        let region_c1 = vec![
                                    (6, 0), (7, 0),
                                    (6, 1), (7, 1), (8, 1),
                            (5, 2), (6, 2),
            (3, 3), (4, 3), (5, 3),
                    (4, 4),
                    (4, 5), (5, 5),
                            (5, 6),
        ].into_iter().collect();
        let cost_c1 = get_cost_to_fence(&region_c1);
        assert_eq!(cost_c1, 392, "Cost of region C (1) is not correct");
        let region_f = vec![
                    (8, 0), (9, 0),
                            (9, 1),
            (7, 2), (8, 2), (9, 2),
            (7, 3), (8, 3), (9, 3),
                    (8, 4)
        ].into_iter().collect();
        let cost_f = get_cost_to_fence(&region_f);
        assert_eq!(cost_f, 180, "Cost of region F is not correct");
        let region_v = vec![
            (0, 2), (1, 2),
            (0, 3), (1, 3),
            (0, 4), (1, 4), (2, 4), (3, 4),
            (0, 5), (1, 5),         (3, 5),
            (0, 6), (1, 6),
        ].into_iter().collect();
        let cost_v = get_cost_to_fence(&region_v);
        assert_eq!(cost_v, 260, "Cost of region V is not correct");
        let region_j = vec![
                    (6, 3),
            (5, 4), (6, 4),
                    (6, 5), (7, 5),
                    (6, 6), (7, 6),
                    (6, 7), (7, 7),
                    (6, 8),
                    (6, 9),
        ].into_iter().collect();
        let cost_j = get_cost_to_fence(&region_j);
        assert_eq!(cost_j, 220, "Cost of region J is not correct");
        let region_c2 = vec![
            (7, 4)
        ].into_iter().collect();
        let cost_c2 = get_cost_to_fence(&region_c2);
        assert_eq!(cost_c2, 4, "Cost of region C (2) is not correct");
        let region_e = vec![
                            (9, 4),
                    (8, 5), (9, 5),
                    (8, 6), (9, 6),
                    (8, 7), (9, 7),
            (7, 8), (8, 8), (9, 8),
            (7, 9), (8, 9), (9, 9),
        ].into_iter().collect();
        let cost_e = get_cost_to_fence(&region_e);
        assert_eq!(cost_e, 234, "Cost of region E is not correct");
        let region_i2 = vec![
                    (2, 5),
                    (2, 6), (3, 6), (4, 6),
            (1, 7), (2, 7), (3, 7), (4, 7), (5, 7),
            (1, 8), (2, 8), (3, 8),         (5, 8),
                            (3, 9)
        ].into_iter().collect();
        let cost_i2 = get_cost_to_fence(&region_i2);
        assert_eq!(cost_i2, 308, "Cost of region I (2) is not correct");
        let region_m = vec![
            (0, 7),
            (0, 8),
            (0, 9), (1, 9), (2, 9)
        ].into_iter().collect();
        let cost_m = get_cost_to_fence(&region_m);
        assert_eq!(cost_m, 60, "Cost of region M is not correct");
        let region_s = vec![
            (4, 8),
            (4, 9), (5, 9),
        ].into_iter().collect();
        let cost_s = get_cost_to_fence(&region_s);
        assert_eq!(cost_s, 24, "Cost of region S is not correct");
    }
    
    #[test]
    fn test_get_discounted_cost_to_fence() {
        let region_r = vec![
            (0, 0), (1, 0), (2, 0), (3, 0),
            (0, 1), (1, 1), (2, 1), (3, 1),
                            (2, 2), (3, 2), (4, 2),
                            (2, 3)
        ].into_iter().collect();
        let cost_r = get_discounted_cost_to_fence(&region_r);
        assert_eq!(cost_r, 120, "Cost of region R is not correct");
        let region_i1 = vec![
            (4, 0), (5, 0),
            (4, 1), (5, 1),
        ].into_iter().collect();
        let cost_i1 = get_discounted_cost_to_fence(&region_i1);
        assert_eq!(cost_i1, 16, "Cost of region I (1) is not correct");
        let region_c1 = vec![
                                    (6, 0), (7, 0),
                                    (6, 1), (7, 1), (8, 1),
                            (5, 2), (6, 2),
            (3, 3), (4, 3), (5, 3),
                    (4, 4),
                    (4, 5), (5, 5),
                            (5, 6),
        ].into_iter().collect();
        let cost_c1 = get_discounted_cost_to_fence(&region_c1);
        assert_eq!(cost_c1, 308, "Cost of region C (1) is not correct");
        let region_f = vec![
                    (8, 0), (9, 0),
                            (9, 1),
            (7, 2), (8, 2), (9, 2),
            (7, 3), (8, 3), (9, 3),
                    (8, 4)
        ].into_iter().collect();
        let cost_f = get_discounted_cost_to_fence(&region_f);
        assert_eq!(cost_f, 120, "Cost of region F is not correct");
        let region_v = vec![
            (0, 2), (1, 2),
            (0, 3), (1, 3),
            (0, 4), (1, 4), (2, 4), (3, 4),
            (0, 5), (1, 5),         (3, 5),
            (0, 6), (1, 6),
        ].into_iter().collect();
        let cost_v = get_discounted_cost_to_fence(&region_v);
        assert_eq!(cost_v, 130, "Cost of region V is not correct");
        let region_j = vec![
                    (6, 3),
            (5, 4), (6, 4),
                    (6, 5), (7, 5),
                    (6, 6), (7, 6),
                    (6, 7), (7, 7),
                    (6, 8),
                    (6, 9),
        ].into_iter().collect();
        let cost_j = get_discounted_cost_to_fence(&region_j);
        assert_eq!(cost_j, 132, "Cost of region J is not correct");
        let region_c2 = vec![
            (7, 4)
        ].into_iter().collect();
        let cost_c2 = get_discounted_cost_to_fence(&region_c2);
        assert_eq!(cost_c2, 4, "Cost of region C (2) is not correct");
        let region_e = vec![
                            (9, 4),
                    (8, 5), (9, 5),
                    (8, 6), (9, 6),
                    (8, 7), (9, 7),
            (7, 8), (8, 8), (9, 8),
            (7, 9), (8, 9), (9, 9),
        ].into_iter().collect();
        let cost_e = get_discounted_cost_to_fence(&region_e);
        assert_eq!(cost_e, 104, "Cost of region E is not correct");
        let region_i2 = vec![
                    (2, 5),
                    (2, 6), (3, 6), (4, 6),
            (1, 7), (2, 7), (3, 7), (4, 7), (5, 7),
            (1, 8), (2, 8), (3, 8),         (5, 8),
                            (3, 9)
        ].into_iter().collect();
        let cost_i2 = get_discounted_cost_to_fence(&region_i2);
        assert_eq!(cost_i2, 224, "Cost of region I (2) is not correct");
        let region_m = vec![
            (0, 7),
            (0, 8),
            (0, 9), (1, 9), (2, 9)
        ].into_iter().collect();
        let cost_m = get_discounted_cost_to_fence(&region_m);
        assert_eq!(cost_m, 30, "Cost of region M is not correct");
        let region_s = vec![
            (4, 8),
            (4, 9), (5, 9),
        ].into_iter().collect();
        let cost_s = get_discounted_cost_to_fence(&region_s);
        assert_eq!(cost_s, 18, "Cost of region S is not correct");
    }
}