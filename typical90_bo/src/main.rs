#![allow(non_snake_case, unused_macros, dead_code)]

use proconio::{input, marker::Chars};

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        s: Chars,
        k: usize
    }
    println!(
        "{}",
        solve(s, k)
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("")
    );
}

fn solve(s: Vec<char>, k: usize) -> Vec<char> {
    let mut now = s;
    for _ in 0..k {
        let mut digit = 0;
        let mut sum = 0;
        for c in now.into_iter().rev() {
            sum += (c.to_digit(10).unwrap() as u64) * 8_u64.pow(digit);
            digit += 1;
        }
        now = vec![];
        while sum != 0 {
            let c = std::char::from_digit((sum % 9) as u32, 10).unwrap();
            let c = if c == '8' { '5' } else { c };
            now.push(c);
            sum /= 9;
        }
        now.reverse();
    }
    if now.is_empty() {
        vec!['0']
    } else {
        now
    }
}
