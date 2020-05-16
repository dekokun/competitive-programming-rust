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

#[derive(PartialEq, Eq)]
enum Result {
    Found,
    Nothing,
}

fn main() {
    let (N, K): (usize, usize) = (read(), read());
    let mut vec: Vec<Vec<usize>> = vec![];
    for _ in 0..N {
        let mut inner = vec![];
        for _ in 0..K {
            inner.push(read());
        }
        vec.push(inner);
    }
    if solve(0, 0, N, &vec) == Result::Found {
        println!("Found")
    } else {
        println!("Nothing")
    }
}

#[allow(dead_code)]
fn solve(xor: usize, depth: usize, max_depth: usize, vec: &[Vec<usize>]) -> Result {
    if depth == max_depth {
        return if xor == 0 {
            Result::Found
        } else {
            Result::Nothing
        };
    }
    if vec[depth]
        .iter()
        .map(|&v| solve(xor ^ v, depth + 1, max_depth, vec))
        .any(|v| v == Result::Found)
    {
        Result::Found
    } else {
        Result::Nothing
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, true);
    }
}
