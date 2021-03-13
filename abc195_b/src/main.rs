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
        a: usize,
        b: usize,
        w: usize,
    }
    println!("{}", solve(a, b, w));

}

fn solve(a: usize, b: usize, w: usize) -> String {
    let w = w * 1000;
    let mut none = true;
    let mut ans = (0, 0);
    for i in 1..=w * 100 {
        if none && a * i <= w && b * i >= w {
            none = false;
            ans.0 = i;
        }
        if !none && a * i > w {
            ans.1 = i - 1;
            break;
        }
    }
    if ans == (0, 0) {
        "UNSATISFIABLE".into()
    } else {
        format!("{} {}", ans.0, ans.1)
    }
}
