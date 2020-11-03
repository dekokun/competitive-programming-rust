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
    let s: Vec<_> = s.chars().collect();
    if s.len() == 1 {
        println!("{}", if s[0] == '8' { "Yes" } else { "No" });
        return;
    }
    if s.len() == 2 {
        let mut str1 = s[0].to_string();
        str1.push(s[1]);
        let num1 = str1.parse::<i32>().unwrap();
        let mut str2 = s[1].to_string();
        str2.push(s[0]);
        let num2 = str2.parse::<i32>().unwrap();
        println!(
            "{}",
            if num1 % 8 == 0 || num2 % 8 == 0 {
                "Yes"
            } else {
                "No"
            }
        );
        return;
    }
    use std::collections::HashMap;
    let mut eights: Vec<HashMap<u32, usize>> = vec![];
    let mut eight_multiple = 8;
    while eight_multiple < 1000 {
        let num = if eight_multiple < 10 {
            eight_multiple * 100
        } else if eight_multiple < 100 {
            eight_multiple * 10
        } else {
            eight_multiple
        };
        let mut map = HashMap::new();
        for c in num.to_string().chars() {
            *map.entry(c.to_digit(10).unwrap()).or_insert(0) += 1;
        }
        eights.push(map);
        eight_multiple += 8;
    }
    let mut s_nums = HashMap::new();
    for c in s {
        *s_nums.entry(c.to_digit(10).unwrap()).or_insert(0) += 1;
    }
    'outer: for eight in eights {
        for (c, val) in eight {
            match s_nums.get(&c) {
                None => continue 'outer,
                Some(&count) => {
                    if count >= val {
                        continue;
                    } else {
                        continue 'outer;
                    }
                }
            }
        }
        println!("Yes");
        return;
    }
    println!("No");
}
