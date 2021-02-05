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
    println!(
        "{}",
        solve(
            n,
            read(),
            read(),
            (0..n).map(|_| (read(), read())).collect()
        )
    );
}

fn solve(_n: usize, a: i32, b: i32, sd: Vec<(String, i32)>) -> String {
    let mut ans = 0;
    for (s, d) in sd {
        let d = if d < a {
            a
        } else if d > b {
            b
        } else {
            d
        };
        ans += match s.as_ref() {
            "West" => -d,
            _ => d,
        }
    }
    if ans == 0 {
        "0".into()
    } else if ans < 0 {
        format!("West {}", -ans)
    } else {
        format!("East {}", ans)
    }
}
