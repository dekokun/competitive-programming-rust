#![allow(non_snake_case, unused_macros)]

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

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

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, true);
    }
}

fn main() {
    let n: usize = read();
    let m: usize = read();
    let ab = (0..m).map(|_| (read(), read())).collect();
    let k: usize = read();

    println!(
        "{}",
        solve(n, m, ab, k, (0..k).map(|_| (read(), read())).collect())
    );
}

fn solve(n: usize, _m: usize, ab: Vec<(usize, usize)>, k: usize, cd: Vec<(usize, usize)>) -> usize {
    let dishes = vec![0; n + 1];
    dfs(dishes, 0, &k, &ab, &cd)
}
fn dfs(
    dishes: Vec<usize>,
    now: usize,
    k: &usize,
    ab: &Vec<(usize, usize)>,
    cd: &Vec<(usize, usize)>,
) -> usize {
    if now >= *k {
        return ab
            .into_iter()
            .filter(|&&(a, b)| dishes[a] >= 1 && dishes[b] >= 1)
            .count();
    }
    let mut dishes1 = dishes.clone();
    dishes1[cd[now].0] += 1;
    let ans1 = dfs(dishes1, now + 1, k, ab, cd);
    let mut dishes2 = dishes;
    dishes2[cd[now].1] += 1;
    let ans2 = dfs(dishes2, now + 1, k, ab, cd);
    std::cmp::max(ans1, ans2)
}
