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
    let vec: Vec<usize> = (0..n).map(|_| read()).collect();
    let max: usize = *vec.iter().max().unwrap();
    let mut digit = 0;
    while 2_usize.pow(digit) < max {
        digit += 1;
    }
    let mut map: Vec<Vec<usize>> = vec![];
    for mut v in vec {
        let mut now = vec![];
        for _ in 0..=digit {
            now.push(v % 2);
            v /= 2;
        }
        map.push(now);
    }
    let mut v2 = vec![];
    for i in 0..=digit {
        let mut sum: usize = 0;
        for j in 0..map.len() {
            sum += map[j][i as usize];
        }
        v2.push(sum);
    }
    let mut ans = vec![0; map.len()];
    for (j, v) in map.into_iter().enumerate() {
        for (i, &val) in v.iter().enumerate() {
            if v2[i] % 2 == 0 {
                ans[j] += 2_usize.pow(i as u32) * val;
            } else {
                let rev = if val == 0 { 1 } else { 0 };
                ans[j] += 2_usize.pow(i as u32) * rev;
            }
        }
    }
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
