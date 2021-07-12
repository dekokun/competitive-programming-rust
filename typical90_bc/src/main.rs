#![allow(non_snake_case, unused_macros, dead_code)]

use proconio::input;

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        a: [usize; n],
    }
    println!("{}", solve(n, p, q, a));
}

fn solve(n: usize, p: usize, q: usize, a: Vec<usize>) -> usize {
    let mut ans = 0;
    for i in 0..n {
        let v = a[i];
        for j in i + 1..n {
            let v = (v * a[j]) % p;
            for k in j + 1..n {
                let v = (v * a[k]) % p;
                for l in k + 1..n {
                    let v = (v * a[l]) % p;
                    for m in l + 1..n {
                        let v = (v * a[m]) % p;
                        if v == q {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    ans
}
