#![allow(non_snake_case)]

use std::io::{stdin, Read};
use std::str::FromStr;
fn read_option<T: FromStr>() -> Option<T> {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok()
}
fn read<T: FromStr>() -> T {
    let opt = read_option();
    opt.expect("failed to parse token")
}

fn main() {
    let H: usize = read();
    let W: usize = read();
    let M: usize = read();
    let mut rows: Vec<usize> = vec![0; H];
    let mut columns: Vec<usize> = vec![0; W];
    let mut subjects = vec![];
    for _ in 0..M {
        let row: usize = read();
        rows[row - 1] += 1;
        let column: usize = read();
        columns[column - 1] += 1;
        subjects.push((row - 1, column - 1));
    }
    let mut rows: Vec<_> = rows.into_iter().enumerate().collect();
    let mut columns: Vec<_> = columns.into_iter().enumerate().collect();
    rows.sort_by_key(|v| v.1);
    columns.sort_by_key(|v| v.1);
    use std::collections::HashSet;
    let mut max_rows = HashSet::new();
    let mut max_columns = HashSet::new();
    for &(i, v) in &rows {
        if v == rows[rows.len() - 1].1 {
            max_rows.insert(i);
        }
    }
    for &(i, v) in &columns {
        if v == columns[columns.len() - 1].1 {
            max_columns.insert(i);
        }
    }
    let mut count = 0;
    for (row, column) in subjects {
        if !(max_rows.contains(&row) && max_columns.contains(&column)) {
            continue;
        }
        count += 1;
    }
    if max_rows.len() * max_columns.len() == count {
        println!(
            "{}",
            rows[rows.len() - 1].1 + columns[columns.len() - 1].1 - 1
        );
    } else {
        println!("{}", rows[rows.len() - 1].1 + columns[columns.len() - 1].1);
    }
}
