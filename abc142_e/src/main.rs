use std::collections::HashMap;
use std::io::*;
use std::str::FromStr;

#[allow(dead_code)]
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

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let opt = read_option();
    opt.expect("failed to parse token")
}

fn main() {
    let n: usize = read();
    let m: usize = read();
    let mut map = HashMap::new();
    for _ in 0..m {
        let a: usize = read();
        let b: usize = read();
        for _ in 0..b {
            let c: usize = read();
            map.entry(c).or_insert(vec![]).push(a);
        }
    }
    let mut ans = 0;
    for i in 1..n + 1 {
        let val = map.get(&i);
        if val.is_none() {
            dbg!(i, val);
            println!("-1");
            return;
        }
        let val = val.unwrap();
        if val.len() == 0 {
            println!("-1");
            return;
        }
        let mut min = 1000000000;
        for v in val {
            min = std::cmp::min(min, *v);
        }
        ans += min;
    }
    println!("{}", ans);
}
