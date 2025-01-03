use std::{cmp::Ordering, collections::{HashMap, HashSet}};

fn main() {
    let (rules, updates) = parse_input("./src/bin/day05/input.txt");
    let (valid, invalid) = split_valid_and_invalid_updates(&rules, &updates);
    // Part 1
    let middle_valid_sum = sum_of_middle_updates(&valid);
    println!("Middle sum: {}", middle_valid_sum);
    // Part 2
    let middle_corrected_invalid_sum = sum_of_middle_updates(&valid_sort(&rules, &invalid));
    println!("Corrected invalid middle sum: {}", middle_corrected_invalid_sum);
}

fn parse_input(file_path: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    // TODO come back to this and try with parser combinators
    let sections: Vec<String> = std::fs::read_to_string(file_path)
        .expect(&format!("Error reading from file path: {}", file_path))
        .split("\n\n")
        .map(|str| str.to_string())
        .collect();
    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    sections[0].split("\n")
        .for_each(|line| {
            let rule = line.split("|")
            .map(|num| num.parse::<usize>().expect(&format!("Invalid input. {} is not a number.", num)))
            .collect::<Vec<usize>>();
            match rules.get_mut(&rule[0]) {
                Some(precedent) => precedent.push(rule[1]),
                None => { rules.insert(rule[0], vec![rule[1]]); }
            };
        });
    let updates = sections[1].split("\n")
        .map(|line| {
            line.split(",")
            .map(|num| num.parse::<usize>().expect(&format!("Invalid input. {} is not a number.", num)))
            .collect::<Vec<usize>>()
        })
        .collect();

    (rules, updates)
}

fn split_valid_and_invalid_updates(rules: &HashMap<usize, Vec<usize>>, updates: &Vec<Vec<usize>>) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    updates.iter()
        .fold((vec![], vec![]), |(mut valid, mut invalid), update| {
            let mut visited: HashSet<usize> = HashSet::new();
            let is_valid = !update.iter().any(|page_num| {
                match rules.get(page_num) {
                    Some(value) => {
                        if value.iter().any(|page| visited.contains(page)) {
                            return true;
                        }
                    }
                    _ => {}
                }
                visited.insert(*page_num);
                false
            });
            if is_valid {
                valid.push(update.clone());
            } else {
                invalid.push(update.clone());
            }
            (valid, invalid)
        })
}

fn valid_sort(rules: &HashMap<usize, Vec<usize>>, updates: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    updates.iter().map(|update| {
        let mut new_update = update.clone();
        new_update.sort_by(|a, b| {
            match rules.get(a) {
                Some(value) => {
                    if value.contains(b) {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                }
                _ => Ordering::Equal
            }
        });
        new_update
    }).collect()
}

fn sum_of_middle_updates(updates: &Vec<Vec<usize>>) -> usize {
    updates.iter()
        .map(|update| update[update.len() / 2])
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {            
        let rules: HashMap<usize, Vec<usize>> = [
            (47, vec![53, 13, 61, 29]),
            (97, vec![13, 61, 47, 29, 53, 75]),
            (75, vec![29, 53, 47, 61, 13]),
            (61, vec![13, 53, 29]),
            (29, vec![13]),
            (53, vec![29, 13])
        ].into_iter().collect();
        let updates = vec![
            vec![75,47,61,53,29],
            vec![97,61,53,29,13],
            vec![75,29,13],
            vec![75,97,47,61,53],
            vec![61,13,29],
            vec![97,13,75,29,47]
        ];

        assert_eq!(parse_input("./src/bin/day05/sample_input.txt"), (rules, updates));
    }

    #[test]
    fn test_split_valid_and_invalid_updates() {
        let rules: HashMap<usize, Vec<usize>> = [
            (47, vec![53, 13, 61, 29]),
            (97, vec![13, 61, 47, 29, 53, 75]),
            (75, vec![29, 53, 47, 61, 13]),
            (61, vec![13, 53, 29]),
            (29, vec![13]),
            (53, vec![29, 13])
        ].into_iter().collect();
        let updates = vec![
            vec![75,47,61,53,29],
            vec![97,61,53,29,13],
            vec![75,29,13],
            vec![75,97,47,61,53],
            vec![61,13,29],
            vec![97,13,75,29,47]
        ];
        let valid_updates = vec![
            vec![75,47,61,53,29],
            vec![97,61,53,29,13],
            vec![75,29,13],
        ];
        let invalid_updates = vec![
            vec![75,97,47,61,53],
            vec![61,13,29],
            vec![97,13,75,29,47]
        ];

        assert_eq!(split_valid_and_invalid_updates(&rules, &updates), (valid_updates, invalid_updates));
    }

    #[test]
    fn test_valid_sort() {
        let rules: HashMap<usize, Vec<usize>> = [
            (47, vec![53, 13, 61, 29]),
            (97, vec![13, 61, 47, 29, 53, 75]),
            (75, vec![29, 53, 47, 61, 13]),
            (61, vec![13, 53, 29]),
            (29, vec![13]),
            (53, vec![29, 13])
        ].into_iter().collect();
        let invalid_updates = vec![
            vec![75,97,47,61,53],
            vec![61,13,29],
            vec![97,13,75,29,47]
        ];
        let sorted_updates = vec![
            vec![97, 75, 47, 61, 53],
            vec![61, 29, 13],
            vec![97, 75, 47, 29, 13]
        ];

        assert_eq!(valid_sort(&rules, &invalid_updates), sorted_updates)
    }

    #[test]
    fn test_sum_of_middle_updates() {
        let valid_updates = vec![
            vec![75,47,61,53,29],
            vec![97,61,53,29,13],
            vec![75,29,13],
        ];
        let corrected_invalid_updates = vec![
            vec![97, 75, 47, 61, 53],
            vec![61, 29, 13],
            vec![97, 75, 47, 29, 13]
        ];

        assert_eq!(sum_of_middle_updates(&valid_updates), 143);
        assert_eq!(sum_of_middle_updates(&corrected_invalid_updates), 123);
    }
}