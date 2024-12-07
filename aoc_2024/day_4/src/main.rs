use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

use ndarray::{s, Array2};

fn get_all_diagonals(board: &Array2<char>) -> (Vec<String>, Vec<String>) {
    let (rows, cols) = board.dim();

    // Top-left to bottom-right diagonals
    let mut left: Vec<String> = Vec::new();
    for k in 0..(rows + cols - 1) {
        let mut diagonal = Vec::new();
        for i in 0..rows {
            let j = k as isize - i as isize;
            if j >= 0 && (j as usize) < cols {
                diagonal.push(board[[i, j as usize]]);
            }
        }
        if !diagonal.is_empty() {
            left.push(diagonal.iter().collect());
        }
    }

    // Top-right to bottom-left diagonals
    let mut right: Vec<String> = Vec::new();
    for k in 0..(rows + cols - 1) {
        let mut diagonal = Vec::new();
        for i in 0..rows {
            let j = (cols as isize - 1) - k as isize + i as isize;
            if j >= 0 && (j as usize) < cols {
                diagonal.push(board[[i, j as usize]]);
            }
        }
        if !diagonal.is_empty() {
            right.push(diagonal.iter().collect());
        }
    }

    (left, right)
}


fn get_manhattan_stinrgs(board: &Array2<char>) -> Vec<String> {

    let mut manhattan_strings = Vec::new();

    let mut rows: Vec<String> = Vec::new();
    for i in 0..board.dim().0 {
        let row = board.slice(s![i, ..]).to_vec();
        rows.push(row.iter().collect());
    }

    let mut cols: Vec<String> = Vec::new();
    for i in 0..board.dim().1 {
        let col = board.slice(s![.., i]).to_vec();
        cols.push(col.iter().collect());
    }

    manhattan_strings.extend(rows);
    manhattan_strings.extend(cols);

    return manhattan_strings;


}


fn parse_matrix(input_file: &str) -> Array2<char> {
    let reader = BufReader::new(File::open(input_file).expect("Bad filepath"));
    let mut char_map: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.unwrap().chars() {
            row.push(c);
        }
        char_map.push(row);
    }
    let num_rows = char_map.len();
    let num_cols = char_map[0].len();
    // why clone :(
    let board: Array2<char> = Array2::from_shape_vec(
        (num_rows, num_cols),
        char_map.iter().flatten().cloned().collect(),
    )
    .unwrap();

    return board;
}

fn solve_p1(input_file: &str) {

    let forward_pattern = Regex::new(r"XMAS").unwrap();
    let backward_pattern = Regex::new(r"SAMX").unwrap();

    let board = parse_matrix(input_file);
    let manhattan_strings = get_manhattan_stinrgs(&board);
    let (left,right)= get_all_diagonals(&board);

    let mut xmas_strings: Vec<String> = Vec::new();
    xmas_strings.extend(manhattan_strings);
    xmas_strings.extend(left);
    xmas_strings.extend(right);


    let mut count = 0;
    for s in xmas_strings {
        forward_pattern.find_iter(&s).for_each(|_| {
            count += 1;
        });
        backward_pattern.find_iter(&s).for_each(|_| {
            count += 1;
        });
    }

    println!("part_1 -> {}", count);
}

fn solve_p2(input_file: &str) {


}

fn main() {
    solve_p1("input/input.txt");
    solve_p1("input/test_input.txt");
    // solve_p2("input/test_input.txt");
    // solve_p2("input/input.txt");

}
