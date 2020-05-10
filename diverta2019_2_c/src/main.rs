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
    let N: usize = read();
    let vec: Vec<isize> = (0..N).map(|_| read()).collect();
    let ans = solve(vec);
    println!("{}", ans.0);
    for v in ans.1 {
        println!("{} {}", v.0, v.1);
    }
}

fn solve(seq: Vec<isize>) -> (usize, Vec<(isize, isize)>) {
    let mut ans = vec![];
    let mut minuses = vec![];
    let mut pluses = vec![];
    let mut zeros = vec![];
    for v in seq {
        match v {
            0 => zeros.push(0),
            a if a < 0 => minuses.push(a),
            a if a > 0 => pluses.push(a),
            _ => panic!(),
        }
    }
    minuses.sort();
    pluses.sort();
    pluses.reverse();

    loop {
        match (zeros.len(), pluses.len(), minuses.len()) {
            (1, 0, 0) | (0, 1, 0) | (0, 0, 1) => break,
            (0, 1, _) => {
                let minus = minuses.pop().unwrap();
                let plus = pluses.pop().unwrap();
                ans.push((plus, minus));
                pluses.push(plus - minus);
            }
            (0, _, 1) => {
                let minus = minuses.pop().unwrap();
                let plus = pluses.pop().unwrap();
                ans.push((minus, plus));
                minuses.push(minus - plus);
            }
            // all minus
            (0, 0, _) => {
                let minus_1 = minuses.pop().unwrap();
                let minus_2 = minuses.pop().unwrap();
                ans.push((minus_1, minus_2));
                pluses.push(minus_1 - minus_2);
            }
            // all plus
            (0, _, 0) => {
                let plus_1 = pluses.pop().unwrap();
                let plus_2 = pluses.pop().unwrap();
                ans.push((plus_1, plus_2));
                minuses.push(plus_1 - plus_2);
            }
            (0, plus_count, minus_count) => {
                let minus = minuses.pop().unwrap();
                let plus = pluses.pop().unwrap();
                if plus_count >= minus_count {
                    ans.push((plus, minus));
                    pluses.push(plus - minus);
                } else {
                    ans.push((minus, plus));
                    minuses.push(minus - plus);
                }
            }
            (1, _, 0) => {
                let plus = pluses.pop().unwrap();
                let zero = zeros.pop().unwrap();
                ans.push((zero, plus));
                minuses.push(-plus);
            }
            (1, 0, _) => {
                let minus = minuses.pop().unwrap();
                let zero = zeros.pop().unwrap();
                ans.push((zero, minus));
                pluses.push(-minus);
            }
            (1, _, _) => {
                let plus = pluses.pop().unwrap();
                let zero = zeros.pop().unwrap();
                ans.push((plus, zero));
                pluses.push(plus);
            }
            (_, _, _) => {
                let zero_1 = zeros.pop().unwrap();
                let zero_2 = zeros.pop().unwrap();
                ans.push((zero_1, zero_2));
                zeros.push(0);
            }
        }
    }
    if pluses.is_empty() {
        (0, ans)
    } else {
        (pluses[0] as usize, ans)
    }
}
