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
    let h: usize = read();
    let w: usize = read();
    println!("{}", solve(h, w, (0..h).map(|_| read()).collect()));
}

fn solve(_h: usize, _w: usize, m: Vec<String>) -> usize {
    let mut before = vec![];
    let mut ans = 0;
    let mut before_cell = None;
    let mut before_blank = None;
    for s in m {
        let mut tmp = vec![];
        for (i, c) in s.chars().enumerate() {
            if c == '#' {
                tmp.push(i);
                if !before.contains(&i) {
                    if let Some(b) = before_cell {
                        if b + 1 != i {
                            ans += 2;
                        }
                    }
                }
                before_cell = Some(i);
            } else {
                if before.contains(&i) {
                    if let Some(b) = before_blank {
                        if b + 1 != i {
                            ans += 2;
                        }
                    }
                }
                before_blank = Some(i);
            }
        }
        before = tmp;
    }
    ans
}
