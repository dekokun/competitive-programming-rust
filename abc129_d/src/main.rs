use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let h: usize = read();
    let w: usize = read();
    let mut result = vec![vec![0; w]; h];
    let mut square = vec![];
    for _ in 0..h {
        let str: String = read();
        let row = str.chars().collect::<Vec<char>>();
        square.push(row);
    }
    for i in 0..h {
        let mut counter = 0;
        let mut start = None;
        for j in 0..w {
            if square[i][j] == '#' {
                if start.is_some() {
                    for s in start.unwrap()..j {
                        result[i][s] += counter;
                    }
                }
                start = None;
                counter = 0;
            } else if j == (w - 1) {
                counter += 1;
                if start.is_none() {
                    start = Some(j);
                }
                for s in start.unwrap()..j + 1 {
                    result[i][s] += counter;
                }
                start = None;
                counter = 0;
            } else {
                counter += 1;
                if start.is_none() {
                    start = Some(j);
                }
            }
        }
    }
    for j in 0..w {
        let mut counter = 0;
        let mut start = None;
        for i in 0..h {
            if square[i][j] == '#' {
                if start.is_some() {
                    for s in start.unwrap()..i {
                        result[s][j] += counter;
                    }
                }
                start = None;
                counter = 0;
            } else if i == (h - 1) {
                counter += 1;
                if start.is_none() {
                    start = Some(i);
                }
                for s in start.unwrap()..i + 1 {
                    result[s][j] += counter;
                }
                start = None;
                counter = 0;
            } else {
                counter += 1;
                if start.is_none() {
                    start = Some(i);
                }
            }
        }
    }
    let mut max = 0;
    for row in result {
        for cell in row {
            max = std::cmp::max(max, cell);
        }
    }
    println!("{}", max - 1);
}
