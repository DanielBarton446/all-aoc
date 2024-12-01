use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let mut first_list = Vec::<i32>::new();
    let mut second_list = Vec::<i32>::new();

    for line in input.lines() {
        let record = line.split_once(",").unwrap();
        first_list.push(record.0.parse().unwrap());
        second_list.push(record.1.parse().unwrap());
    }
    first_list.sort();
    second_list.sort();

    let part_1_total = first_list
        .iter()
        .zip(second_list.iter())
        .fold(0, |acc, (x, y)| acc + (x - y).abs());

    println!("part_1 => {}", part_1_total);

     
    let second_frequency_count = second_list.iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    let part_2_total = first_list.iter().fold(0, |acc, x| {
        if second_frequency_count.contains_key(x) {
            acc + (x * second_frequency_count.get(x).unwrap())
        } else {
            acc
        }
    });

    println!("part_2 => {}", part_2_total);
}
