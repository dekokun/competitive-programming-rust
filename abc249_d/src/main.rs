#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::HashMap;

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
        a: [usize; n]
    }
    println!("{}", solve(a));
}

fn solve(mut a: Vec<usize>) -> usize {
    a.sort();
    debug!(a);
    let mut m = HashMap::new();
    for &v in &a {
        let e = m.entry(v).or_insert(0);
        *e += 1;
    }
    let mut ans = 0;
    let max = a[a.len() - 1];
    for (&key, &count) in &m {
        let mut n = 1;
        while key * n <= max {
            if m.contains_key(&(key * n)) && m.contains_key(&n) {
                ans += count * m[&(key * n)] * m[&n];
                // debug!(key, n, key * n, count * m[&key * n] * m[&n]);
            }
            n += 1;
        }
    }
    ans
}
