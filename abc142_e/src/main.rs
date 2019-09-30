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
    let mut keys: Vec<(usize, usize)> = vec![];
    for _ in 0..m {
        let a: usize = read();
        let b: usize = read();
        let mut key = 0;
        for _ in 0..b {
            let c: u32 = read();
            key |= (2 as usize).pow(c - 1);
        }
        keys.push((key, a));
    }
    const INF: usize = 1001001001;
    let mut dp = vec![INF; (2 as usize).pow(n as u32)];
    dp[0] = 0;
    for i in 0..(2 as usize).pow(n as u32) {
        if dp[i] == INF {
            continue;
        }
        for key in keys.iter() {
            let k = i | key.0;
            dp[k] = std::cmp::min(dp[k], dp[i] + key.1);
        }
    }
    let v = dp.pop().unwrap();
    println!("{}", if v != INF {
        v as i32
    } else {
        -1
    });
}
