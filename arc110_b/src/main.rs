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
    let _n: u64 = read();
    let t: String = read();
    let repeat = 10_u64.pow(10);
    if t.len() == 1 {
        let ans = match t.chars().next().unwrap() {
            '0' => {
                repeat
            },
            '1' => {
                repeat * 2
            },
            _ => {
                0
            }
        };
        println!("{}", ans);
        return
    }
    if t.len() == 2 {
        let ans = match t.as_ref() {
            "11" | "10" => {
                repeat
            },
            "01" => {
                repeat - 1
            },
            _ => {
                0
            }
        };
        println!("{}", ans);
        return
    }
    if t.len() == 3 {
        let ans = match t.as_ref() {
            "110" => {
                repeat
            },
            "101" | "011" => {
                repeat - 1
            },
            _ => {
                0
            }
        };
        println!("{}", ans);
        return
    }
    // 110110110
    let start = match t[0..2].as_ref() {
        "11" => {
            ""
        },
        "10" => {
            "1"
        },
        "01" => {
            "11"
        },
        _ => {
            println!("{}", 0);
            return
        }
    };
    let t = start.to_string() + t.as_ref();
    let ok: Vec<_> = "110".repeat(t.len() + 3 / 3 ).chars().collect();
    for (i, c) in t.chars().enumerate() {
        if ok[i] != c {
            println!("{}", 0);
            return;
        }
    }
    // 110110110
    // 110 => 3
    // 1101 => 2
    // 11011 => 2
    // 110110 => 2
    // 1101101 => 1
    println!("{}", repeat - ((t.len() - 1) / 3) as u64);
}
