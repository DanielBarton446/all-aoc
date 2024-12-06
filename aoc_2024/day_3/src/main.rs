use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use regex::Regex;

fn parse_string_from_file(path: &Path) -> Result<Vec<String>, std::io::Error> {
    let reader = BufReader::new(File::open(path)?);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.expect("Failed to read a line ffs"));
    }

    return Ok(lines);
}

fn solve_part1(input: Vec<String>) -> Result<usize, regex::Error> {
    let pattern = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|do\(\)|don't\(\)")?;

    let mut mul_strings: Vec<String> = Vec::new();
    let mut flag = true;
    for line in input {
        pattern.find_iter(&line).for_each(|needle| {
            if needle.as_str() == "don't()" {
                flag = false;
            } else if needle.as_str() == "do()" {
                flag = true;
            }
            if flag {
                mul_strings.push(needle.as_str().to_string());
            }
        })
    }

    Ok(solve_part1(mul_strings)?)
}

fn solve_part2(input: Vec<String>) -> Result<usize, regex::Error> {
    let mut parse_muls: Vec<String> = Vec::new();
    for line in input {
        let dont_split: Vec<&str> = line.split("don't()").collect();
        let (valid, fetch_dos) = dont_split.split_at(1);
        for entity in valid {
            parse_muls.push(entity.to_string());
        }

        for chunk in fetch_dos {
            match chunk.split_once("do()") {
                Some((_, do_after)) => {
                    parse_muls.push(do_after.to_string());
                }
                _ => continue,
            }
        }
    }

    return Ok(solve_part1(parse_muls)?);
}

fn main() {
    println!("Hello, world!");

    println!(
        "part_1 => {}",
        solve_part1(parse_string_from_file(Path::new("input/input.txt")).expect("not a file dude"))
            .unwrap()
    );
    println!(
        "part_2 => {}",
        solve_part2(parse_string_from_file(Path::new("input/input.txt")).expect("not a file dude"))
            .unwrap()
    );
}
