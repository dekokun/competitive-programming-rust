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

fn main() {
    let n: usize = read();

    println!(
        "{}",
        if let Some(a) = solve((0..n).map(|_| (read(), read(), read())).collect()) {
            a.to_string()
        } else {
            "-1".into()
        }
    );
}

fn solve(apx: Vec<(usize, usize, usize)>) -> Option<usize> {
    let mut ans = std::usize::MAX;
    for (a, p, x) in apx {
        if x > a {
            ans = ans.min(p);
        }
    }
    if ans == std::usize::MAX {
        None
    } else {
        Some(ans)
    }
}
