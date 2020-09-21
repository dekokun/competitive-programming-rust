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
    let n: u64 = read();
    let mut x: u64 = read();
    let m: u64 = read();
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut i = 0;
    let mut cumsum = vec![0; m as usize];
    let mut sum = 0;
    let mut vec = vec![];
    while !map.contains_key(&x) {
        map.insert(x, i);
        vec.push(x);
        i += 1;
        sum += x;
        cumsum[i] = sum;
        x = x.pow(2) % m;
        if i == n as usize {
            println!("{}", sum);
            return;
        }
    }
    let loop_sum = cumsum[i] - cumsum[map[&x]];
    let inner_loop_count = i - map[&x];
    let loop_start_sum = cumsum[map[&x]];
    dbg!(
        loop_sum,
        inner_loop_count,
        loop_start_sum,
        (n - i as u64 + 1) / inner_loop_count as u64,
        (n - map[&x] as u64 + 1) % inner_loop_count as u64
    );
    let mut ans = loop_sum * ((n - i as u64 + 1) / inner_loop_count as u64) + loop_start_sum;
    for v in 0..((n - map[&x] as u64) % inner_loop_count as u64) {
        ans += (vec[map[&x] + v as usize]) as u64;
    }
    println!("{}", ans)
}
