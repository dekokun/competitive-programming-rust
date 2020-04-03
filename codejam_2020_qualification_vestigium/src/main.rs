#![allow(non_snake_case)]

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

fn main() {
    let T: usize = read();
    for t in 1..T + 1 {
        let N: usize = read();
        let mut matrix: Vec<Vec<usize>> = vec![vec![0; N]; N];
        for i in 0..N {
            for j in 0..N {
                matrix[i][j] = read();
            }
        }
        let mut trace = 0;
        for i in 0..N {
            trace += matrix[i][i];
        }

        let mut ans1 = 0;
        for i in 0..N {
            use std::collections::HashSet;
            let mut set: HashSet<usize> = HashSet::new();
            for j in 0..N {
                set.insert(matrix[i][j]);
            }
            if set.len() != N {
                ans1 += 1;
            }
        }
        let mut ans2 = 0;
        for i in 0..N {
            use std::collections::HashSet;
            let mut set: HashSet<usize> = HashSet::new();
            for j in 0..N {
                set.insert(matrix[j][i]);
            }
            if set.len() != N {
                ans2 += 1;
            }
        }

        println!("Case #{}: {} {} {}", t, trace, ans1, ans2);
    }
}
