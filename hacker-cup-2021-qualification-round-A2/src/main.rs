#![allow(non_snake_case, unused_macros, dead_code)]

use proconio::{input, marker::Chars};

// https://maguro.dev/debug-macro/ $B$+$i(B
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        T: usize,
    }
    for t in 1..=T {
        input! {
            s: Chars,
            n: usize,
            ss: [Chars; n]
        }
        println!("Case #{}: {}", t, solve(s, ss));
    }
}

fn solve(s: Vec<char>, ss: Vec<Vec<char>>) -> i32 {
    let inf = 10_000;
    let mut graph = vec![vec![inf; 26]; 26];
    for cc in ss {
        graph[index(cc[0])][index(cc[1])] = 1;
    }
    for i in 0..26 {
        graph[i][i] = 0;
    }
    // Floyd–Warshall Algorithm
    for k in 0..26 {
        for i in 0..26 {
            for j in 0..26 {
                if graph[i][j] > graph[i][k] + graph[k][j] {
                    graph[i][j] = graph[i][k] + graph[k][j];
                }
            }
        }
    }
    let mut ans = inf;
    'outer: for i in 0..26 {
        let mut tmp = 0;
        for &c in &s {
            if graph[index(c)][i] == inf {
                continue 'outer;
            }
            tmp += graph[index(c)][i];
        }
        ans = ans.min(tmp);
    }
    if ans == inf {
        -1
    } else {
        ans
    }
}

fn index(c: char) -> usize {
    (c as u8 - b'A') as usize
}
