#![allow(non_snake_case, unused_macros, dead_code)]

use proconio::input;

// https://maguro.dev/debug-macro/
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a: [usize; n]
    }
    println!("{}", solve(x, y, a));
}

fn solve(x: usize, y: usize, a: Vec<usize>) -> u64 {
    let mut tmp = vec![];
    let mut ans = 0;
    for v in a {
        if y <= v && v <= x {
            tmp.push(v);
            continue;
        }
        ans += f(x, y, &tmp);
        tmp = vec![];
    }
    ans += f(x, y, &tmp);
    ans
}

fn f(x: usize, y: usize, a: &Vec<usize>) -> u64 {
    let mut ret = 0;
    // 包除原理
    // 全体
    ret += (a.len() as u64 * (a.len() + 1) as u64) / 2;
    // x が含まれない
    let mut tmp = 0;
    for &v in a {
        if v != x {
            tmp += 1;
            continue;
        }
        ret -= (tmp * (tmp + 1)) / 2;
        tmp = 0;
    }
    ret -= (tmp * (tmp + 1)) / 2;
    // y が含まれない
    let mut tmp = 0;
    for &v in a {
        if v != y {
            tmp += 1;
            continue;
        }
        ret -= (tmp * (tmp + 1)) / 2;
        tmp = 0;
    }
    ret -= (tmp * (tmp + 1)) / 2;
    // x, y 双方が含まれない
    let mut tmp = 0;
    for &v in a {
        if v != x && v != y {
            tmp += 1;
            continue;
        }
        ret += (tmp * (tmp + 1)) / 2;
        tmp = 0;
    }
    ret += (tmp * (tmp + 1)) / 2;
    ret
}
