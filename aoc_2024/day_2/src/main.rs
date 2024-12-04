use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn is_safe(report: Vec<i32>) -> bool {
    if report.is_empty() {
        unreachable!("We should never be able to provide empty reports!")
    }

    if report.len() == 1 {
        return true;
    }

    // positive => increasing
    // negative => decreasing
    let mut last = report[0];
    let direction = report[1] - report[0];

    if direction == 0 {
        return false;
    }

    for level in report.iter().skip(1) {
        let diff = level - last;
        if diff == 0 {
            return false;
        } else if diff.is_positive() != direction.is_positive() {
            return false;
        } else if diff.abs() > 3 {
            return false;
        }
        last = *level;
    }

    return true;
}

fn parse_report(input: &str) -> Vec<i32> {
    return input
        .split(" ")
        .map(|x| str::parse::<i32>(x).unwrap())
        .collect();
}

fn solve_part1(input: Vec<String>) -> usize {
    input
        .into_iter()
        .map(|report_string| is_safe(parse_report(&report_string)))
        .filter(|&is_safe| is_safe)
        .count()
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input/input.txt")?;
    let input: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("failed to read line!"))
        .collect();

    println!("p1 => {}", solve_part1(input));
    // println!("p1 => {}", solve_part2());
    Ok(())
}

#[cfg(test)]
mod day01_tests {

    use super::*;

    #[test]
    fn p1_sample() {
        let sample_input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        let reports: Vec<Vec<i32>> = sample_input
            .split('\n')
            .map(|record| parse_report(record))
            .collect();
        let results: Vec<bool> = reports.into_iter().map(|record| is_safe(record)).collect();
        assert_eq!(results, [true, false, false, false, false, true])
    }
}
