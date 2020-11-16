#![allow(non_snake_case)]
use std::collections::HashSet;

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
    let k: usize = read();
    let mut times: Vec<Vec<usize>> = vec![];
    for _ in 0..n {
        let mut row = vec![];
        for _ in 0..n {
            row.push(read());
        }
        times.push(row);
    }
    // 1(0)から始まって1に戻るので、0は除く
    let mut remain: HashSet<_> = (1..n).collect();
    let ans = dfs(0, &mut remain, 0, &times, k, n);
    println!("{}", ans)
}

fn dfs(
    now: usize,
    remain: &HashSet<usize>,
    time: usize,
    times: &Vec<Vec<usize>>,
    k: usize,
    n: usize,
) -> usize {
    if remain.len() == 0 && time + times[now][0] == k {
        return 1;
    }
    if remain.len() == 0 && time + times[now][0] != k {
        return 0;
    }
    if time + times[now][0] >= k {
        return 0;
    }
    let mut ans = 0;
    for &v in remain {
        let time = time + times[now][v] + if remain.len() == n { times[0][v] } else { 0 };
        let mut remain2 = remain.clone();
        remain2.remove(&v);
        ans += dfs(v, &mut remain2, time, times, k, n);
    }
    ans
}
