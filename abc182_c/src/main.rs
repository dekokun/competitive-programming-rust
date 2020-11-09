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
    let s: String = read();
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert(0, 0);
    map.insert(1, 0);
    map.insert(2, 0);
    let mut sum = 0;
    for c in s.chars() {
        let num = c.to_digit(10).unwrap() % 3;
        *map.entry(num).or_insert(0) += 1;
        sum += num;
    }
    let one = *map.get(&1).unwrap();
    let two = *map.get(&2).unwrap();
    if sum % 3 == 0 {
        println!("0");
    } else if sum % 3 == 1 {
        if one >= 1 {
            if s.len() > 1 {
                println!("1");
            } else {
                println!("-1");
            }
        } else if two >= 2 && s.len() > 2 {
            println!("2");
        } else {
            println!("-1");
        }
    } else if sum % 3 == 2 {
        if two >= 1 {
            if s.len() > 1 {
                println!("1");
            } else {
                println!("-1");
            }
        } else if one >= 2 && s.len() > 2 {
            println!("2");
        } else {
            println!("-1");
        }
    }
    // use std::cmp::{max, min};
    // let diff = max(one, two) - min(one, two);
    // if diff % 3 != 0 {
    //     if (*one == 0 || *two == 0) && one + two == s.len() as i32 {
    //         println!("-1");
    //     } else {
    //         println!("{}", diff % 3);
    //     }
    // } else {
    //     println!("0");
    // }
}
