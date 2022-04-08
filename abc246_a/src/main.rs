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
        xy: [(isize, isize); 3],
    }
    let ans = solve(xy);
    println!("{} {}", ans.0, ans.1);
}

fn solve(mut xy: Vec<(isize, isize)>) -> (isize, isize) {
    xy.sort_by_key(|a| a.0);
    debug!(xy);
    let x_min = xy[0].0;
    let x_max = xy[2].0;
    xy.sort_by_key(|a| a.1);
    let y_min = xy[0].1;
    let y_max = xy[2].1;
    for (x, y) in vec![
        (x_min, y_min),
        (x_min, y_max),
        (x_max, y_min),
        (x_max, y_max),
    ] {
        if xy.contains(&(x, y)) {
            continue;
        }
        return (x, y);
    }
    panic!()
}
