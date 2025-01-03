
fn main() {
    let reports = parse_input("./src/bin/day02/input.txt");
    // Part 1
    let counts = count_safe_reports(&reports);
    println!("{}", counts);
    // Part 2
    let counts_with_dampener = count_safe_reports_dampened(&reports);
    println!("{}", counts_with_dampener);
}

fn parse_input(file_path: &str) -> Vec<Vec<i32>> {
    let input_string = std::fs::read_to_string(file_path)
        .expect(&format!("Error reading file path {}", file_path));
    input_string.split("\n").map(|line| {
        line.split_whitespace().map(|item| {
            item.parse::<i32>().expect(&format!("Invalid input. {} is not a number.", item))
        }).collect()
    }).filter(|report: &Vec<i32>| report.len() > 0)
    .collect()
}

fn count_safe_reports(reports: &Vec<Vec<i32>>) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

/**
 * Sliding window technique to determine if each level transition is safe.
 */
fn is_safe(report: &Vec<i32>) -> bool {
    let differences: Vec<i32> = report.windows(2).map(|window| window[0] - window[1]).collect();
    (differences.iter().all(|difference| difference.is_positive() && (1..=3).contains(difference))) ||
    (differences.iter().all(|difference| difference.is_negative() && (-3..=-1).contains(difference)))
}

fn count_safe_reports_dampened(reports: &Vec<Vec<i32>>) -> usize {
    reports.iter().filter(|report| is_safe_dampened(report)).count()
}

fn is_safe_dampened(report: &Vec<i32>) -> bool {
    report.iter().enumerate().fold(Vec::<Vec<i32>>::new(), |mut acc, (index, _)| {
        let mut report_iteration = report.clone();
        report_iteration.remove(index);
        acc.push(report_iteration);
        acc
    }).iter().any(|iteration| is_safe(iteration))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let file_path = "./src/bin/day02/sample_input.txt";
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9]
        ];
        assert_eq!(parse_input(file_path), reports);
    }

    #[test]
    fn test_is_safe() {
        let report1 = vec![7, 6, 4, 2, 1];
        let report2 = vec![1, 2, 7, 8, 9];
        let report3 = vec![9, 7, 6, 2, 1];
        let report4 = vec![1, 3, 2, 4, 5];
        let report5 = vec![8, 6, 4, 4, 1];
        let report6 = vec![1, 3, 6, 7, 9];

        assert_eq!(is_safe(&report1), true, "Report 1");
        assert_eq!(is_safe(&report2), false, "Report 2");
        assert_eq!(is_safe(&report3), false, "Report 3");
        assert_eq!(is_safe(&report4), false, "Report 4");
        assert_eq!(is_safe(&report5), false, "Report 5");
        assert_eq!(is_safe(&report6), true, "Report 6");
    }

    #[test]
    fn test_count_safe_reports() {
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9]
        ];

        assert_eq!(count_safe_reports(&reports), 2);
    }

    #[test]
    fn test_is_safe_dampened() {
        let report1 = vec![7, 6, 4, 2, 1];
        let report2 = vec![1, 2, 7, 8, 9];
        let report3 = vec![9, 7, 6, 2, 1];
        let report4 = vec![1, 3, 2, 4, 5];
        let report5 = vec![8, 6, 4, 4, 1];
        let report6 = vec![1, 3, 6, 7, 9];

        assert_eq!(is_safe_dampened(&report1), true, "Report 1");
        assert_eq!(is_safe_dampened(&report2), false, "Report 2");
        assert_eq!(is_safe_dampened(&report3), false, "Report 3");
        assert_eq!(is_safe_dampened(&report4), true, "Report 4");
        assert_eq!(is_safe_dampened(&report5), true, "Report 5");
        assert_eq!(is_safe_dampened(&report6), true, "Report 6");
    }

    #[test]
    fn test_count_safe_reports_dampened() {
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9]
        ];

        assert_eq!(count_safe_reports_dampened(&reports), 4);
    }
}