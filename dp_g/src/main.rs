use std::collections::HashSet;
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
    let mut vec: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for _ in 0..m {
        let x: usize = read();
        let y: usize = read();
        vec[x - 1].insert(y - 1);
    }
    let mut incoming_degrees = vec![Some(0); n];
    for set in &vec {
        for s in set {
            incoming_degrees[*s] = Some(incoming_degrees[*s].unwrap() + 1);
        }
    }
    let mut sorted = vec![];
    let mut zero_set = vec![];
    for (i, v) in incoming_degrees.clone().iter().enumerate() {
        if *v == Some(0) {
            zero_set.push(i);
            incoming_degrees[i] = None;
        }
    }
    // 多分cloneのせいでTLEになってる。n^2でも計算量的には間に合わない。正しくトポロジカルソートを実装しよう
    // 入次数が0のものを数えるのに毎回全部見る必要はなくて、初回だけ全部みてあとはソートしたものから出てるものだけ見れば良い
    while !zero_set.is_empty() {
        let s = zero_set.pop().unwrap();
        sorted.push(s);
        for &v in &vec[s] {
            if incoming_degrees[v].is_none() {
                continue;
            }
            let incoming_count = std::cmp::max(0, incoming_degrees[v].unwrap() - 1);
            incoming_degrees[v] = Some(incoming_count);
            if incoming_count == 0 {
                zero_set.push(v);
            }
        }
    }
    let mut dp = vec![0; n];
    for v in sorted {
        for &v2 in &vec[v] {
            dp[v2] = std::cmp::max(dp[v2], dp[v] + 1);
        }
    }
    println!("{}", dp.iter().max().unwrap());
}
