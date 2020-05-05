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
    println!("{}", solve(N));
}

fn solve(n: usize) -> String {
    let max = 55555;
    let mut vec: Vec<usize> = (0..max).collect();
    let mut ans_primes: Vec<usize> = vec![];
    for i in 0..max {
        if i < 2 {
            vec[i] = 0;
            continue;
        }
        let val = vec[i];
        if val == 0 {
            continue;
        }
        if val % 5 == 1 {
            ans_primes.push(val);
            if ans_primes.len() == n {
                break;
            }
        }
        let mut j = 1;
        while val * j < max {
            vec[val * j] = 0;
            j += 1;
        }
    }
    ans_primes
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!("11 31 41 61 71", solve(5));
    }
}
