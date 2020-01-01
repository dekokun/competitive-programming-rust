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
    let A: usize = read();
    let B: usize = read();
    println!("{}", lcm(A, B));
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 15), 3);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(5, 10), 10)
    }
}
