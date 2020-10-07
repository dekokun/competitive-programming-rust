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
    let n: usize = read();
    use std::collections::HashMap;
    let mut odds = HashMap::new();
    let mut evens = HashMap::new();
    let mut vec = vec![];
    for i in 1..=n {
        let v: usize = read();
        vec.push(v);
        if i % 2 == 0 {
            let entry = evens.entry(v).or_insert(0);
            *entry += 1;
        } else {
            let entry = odds.entry(v).or_insert(0);
            *entry += 1;
        }
    }
    let mut odds: Vec<_> = odds.into_iter().collect();
    let mut evens: Vec<_> = evens.into_iter().collect();
    odds.sort_by_key(|v| v.1);
    evens.sort_by_key(|v| v.1);
    let odd_1 = odds[odds.len() - 1].0;
    let even_1 = evens[evens.len() - 1].0;
    let odd_2 = if odds.len() >= 2 {
        odds[odds.len() - 2].0
    } else {
        odd_1
    };
    let even_2 = if evens.len() >= 2 {
        evens[evens.len() - 2].0
    } else {
        even_1
    };

    let mut ans = n;
    let mut change = false;
    for &(odd, even) in &[
        (odd_1, even_1),
        (odd_1, even_2),
        (odd_2, even_1),
        (odd_2, even_2),
    ] {
        let mut tmp_ans = 0;
        if odd == even {
            continue;
        }
        change = true;
        for (i, &v) in vec.iter().enumerate() {
            if i % 2 == 0 {
                if odd != v {
                    tmp_ans += 1;
                }
            } else {
                if even != v {
                    tmp_ans += 1;
                }
            }
        }
        ans = std::cmp::min(tmp_ans, ans);
    }
    if !change {
        ans = n / 2;
    }
    println!("{}", ans);
    return;
}
