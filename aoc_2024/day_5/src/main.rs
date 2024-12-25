use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Error},
};

fn parse_input(filepath: &str) -> Result<(HashMap<usize, Vec<usize>>, Vec<Vec<usize>>), Error> {
    let reader = BufReader::new(std::fs::File::open(filepath).unwrap());

    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut data: Vec<Vec<usize>> = Vec::new();

    for line in reader.lines() {
        let trimmed_line = line?.trim().to_string();
        if trimmed_line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = trimmed_line.split('|').collect();
        if parts.len() == 2 {
            rules
                .entry(parts[0].parse().unwrap())
                .or_default()
                .push(parts[1].parse().unwrap());
        } else {
            data.push(
                trimmed_line
                    .split(',')
                    .map(|x| x.parse().unwrap())
                    .collect(),
            );
        }
    }

    return Ok((rules, data));
}

fn solve_p1(rules: &HashMap<usize, Vec<usize>>, data: &Vec<Vec<usize>>) -> usize {
    // todo:
    // 1. read data left to right
    // 2. insert the <val>:<position> into a hashmap of visited
    // 3. upon visiting a value, for each rule in rules[val], check if the rule is not satisfied
    let mut total = 0;

    for record in data {
        let mut valid_record = true;
        let mut visited: HashMap<usize, usize> = HashMap::new();
        for (i, val) in record.iter().enumerate() {
            let res = visited.insert(*val, i);
            if res.is_some() {
                panic!("We somehow had already seen page {} in this record", val);
            }
            if let Some(pages_to_check) = rules.get(val) {
                for page in pages_to_check {
                    let visited_page_position = visited.get(page);
                    if visited_page_position.is_none(){
                        continue
                    }
                    if *visited_page_position.unwrap() < i {
                        // at this point we need to skip this record
                        valid_record = false;
                        break;
                    }
                }
            }
            if !valid_record {
                break
            }
        }
        if !valid_record {
            continue
        }

        // now we know we have a valid record, 
        total += record[record.len() / 2 ]

    }

    return total;
}

fn main() -> Result<(), Error> {
    let (rules, data) = parse_input("input/input.txt")?;
    println!("part 1 -> {}", solve_p1(&rules, &data));

    // println!("Hello, world!");
    return Ok(());
}
