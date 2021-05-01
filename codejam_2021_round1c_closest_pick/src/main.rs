#![allow(non_snake_case, unused_macros, dead_code)]

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    let T: usize = read();
    for t in 1..=T {
        let m: usize = read();
        println!(
            "Case #{}: {}",
            t,
            solve((0..m).map(|_| (read(), read())).collect())
        );
    }
}

fn solve(n: usize) -> usize {}
