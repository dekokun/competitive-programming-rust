#![allow(non_snake_case, unused_macros)]

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

use std::str::FromStr;
use std::{
    collections::HashSet,
    io::{stdin, Read},
};
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
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test1() {
        for v in 1..1000 {
            let ans = solve(v);
            assert_eq!(
                ans.len(),
                ans.into_iter().collect::<HashSet<String>>().len()
            )
        }
    }
}

fn main() {
    let n: usize = read();
    println!("{}", solve(n).join("\n"));
}

fn solve(n: usize) -> Vec<String> {
    let mut ans = HashSet::new();
    let mut sum = 0;
    if n == 1 {
        return vec!["1".into()];
    }
    for v in 1..=n {
        sum += v;
        ans.insert(v);
        if sum >= n {
            ans.remove(&(sum - n));
            break;
        }
    }
    ans.into_iter().map(|s| s.to_string()).collect()
}
