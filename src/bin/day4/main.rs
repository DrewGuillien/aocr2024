use std::f32::consts::PI;

fn main() {
    let grid = parse_input("./src/bin/day4/input.txt");
    // Part 1
    let xmas_count = count_of_xmas(&grid);
    println!("XMAS count: {}", xmas_count);
    // Part 2
    let x_mas_count = count_of_x_mas(&grid);
    println!("X MAS count: {}", x_mas_count);
}

fn parse_input(file_path: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(file_path)
        .expect(&format!("Error reading from file path: {}", file_path))
        .split("\n")
        .filter(|slice| slice.len() > 0)
        .map(|slice| slice.chars().collect())
        .collect()
}

fn get_offset(direction: u8) -> (i32, i32) {
    let x = ((f32::from(direction) * PI / 4f32).cos().round()) as i32;
    let y = ((f32::from(direction) * PI / 4f32).sin().round()) as i32;
    (x, y)
}

fn count_of_xmas(grid: &Vec<Vec<char>>) -> usize {
    let word: Vec<char> = "XMAS".chars().collect();
    // For every row
    grid.iter().enumerate().map(|(y, row)| {
        // For every character in the row
        row.iter().enumerate().map(|(x, _)| {
            // Check all 8 directions
            //       5   6   7
            //         \ | /
            //      4 ---*--- 0
            //         / | \
            //       3   2   1
            (0..8u8).filter(|direction| {
                // Check all characters in that direction
                (0..word.len()).all(|index| {
                    // Calculate the change in x and y
                    // https://en.wikipedia.org/wiki/Unit_circle
                    let (x_offset, y_offset) = get_offset(*direction);
                    let delta_x = index as i32 * x_offset;
                    let delta_y = index as i32 * y_offset;
                    let new_x = (x as i32 + delta_x) as usize;
                    let new_y = (y as i32 + delta_y) as usize;
                    // check bounds
                    (0..row.len()).contains(&new_x) && (0..grid.len()).contains(&new_y) &&
                    // check the search word
                    grid[new_y][new_x] == word[index]
                })
            }).count()
        }).sum::<usize>()
    }).sum()
}

fn count_of_x_mas(grid: &Vec<Vec<char>>) -> usize {
    // 2D sliding window
    (0..grid.len() - 2).map(|y| {
        (0..grid[y].len() - 2).filter(|x| {
            grid[y+1][x+1] == 'A' &&
            ((grid[y][*x] == 'M' && grid[y+2][x+2] == 'S') || (grid[y][*x] == 'S' && grid[y+2][x+2] == 'M')) &&
            ((grid[y+2][*x] == 'M' && grid[y][x+2] == 'S') || (grid[y+2][*x] == 'S' && grid[y][x+2] == 'M'))
        }).count()
    }).sum()
}

// For debugging
fn _to_string(direction: u8) -> String {
    match direction {
        0 => "right".to_string(),
        1 => "down right".to_string(),
        2 => "down".to_string(),
        3 => "down left".to_string(),
        4 => "left".to_string(),
        5 => "up left".to_string(),
        6 => "up".to_string(),
        7 => "up right".to_string(),
        _ => "invalid direction".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let grid = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X']
        ];
        assert_eq!(parse_input("./src/bin/day4/sample_input.txt"), grid);
    }

    #[test]
    fn test_count_of_xmas() {
        let grid = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X']
        ];

        assert_eq!(count_of_xmas(&grid), 18);
    }

    #[test]
    fn test_count_of_x_mas() {
        let grid = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X']
        ];

        assert_eq!(count_of_x_mas(&grid), 9);
    }

    #[test]
    fn test_get_offset() {
        assert_eq!(get_offset(0), (1, 0));
        assert_eq!(get_offset(1), (1, 1));
        assert_eq!(get_offset(2), (0, 1));
        assert_eq!(get_offset(3), (-1, 1));
        assert_eq!(get_offset(4), (-1, 0));
        assert_eq!(get_offset(5), (-1, -1));
        assert_eq!(get_offset(6), (0, -1));
        assert_eq!(get_offset(7), (1, -1));
    }
}