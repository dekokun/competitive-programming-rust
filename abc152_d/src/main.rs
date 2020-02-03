#![allow(non_snake_case)]

use std::cmp::Ordering;
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
    let mut vec: Vec<usize> = vec![0; N + 1];
    for i in 1..N + 1 {
        let right_digit = i % 10;
        let (left_digit, num_of_digits) = left_digit_and_num_of_digits(i);
        let reverse = if num_of_digits == 0 {
            left_digit
        } else {
            right_digit * 10 + left_digit
        };
        let normal = if num_of_digits == 0 {
            left_digit
        } else {
            left_digit * 10 + right_digit
        };
        vec[i] = match normal.cmp(&reverse) {
            Ordering::Greater if right_digit == 0 => vec[i - 1],
            Ordering::Greater => vec[i - 1] + 2,
            Ordering::Equal if i < 10 => vec[i - 1] + 1,
            Ordering::Equal => vec[i - 1] + 3,
            Ordering::Less => vec[i - 1],
        };
    }
    println!("{}", vec[N]);
}

fn left_digit_and_num_of_digits(i: usize) -> (usize, usize) {
    let mut i = i;
    let mut num_of_digits = 0;
    while i >= 10 {
        i /= 10;
        num_of_digits += 1;
    }
    (i, num_of_digits)
}
