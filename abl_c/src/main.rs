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
    let (n, m): (usize, usize) = (read(), read());
    let mut uft = UFT::new(n);
    for _ in 0..m {
        let a: usize = read();
        let b: usize = read();
        uft.merge(a - 1, b - 1);
    }
    use std::collections::HashSet;
    let mut roots = HashSet::new();
    for i in 0..n {
        let root = uft.root(i);
        roots.insert(root);
    }
    println!("{}", roots.len() - 1)
}

/// Union Find Tree
pub struct UFT {
    pub par: Vec<usize>,
    pub rank: Vec<usize>,
}
impl UFT {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        UFT {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    #[allow(dead_code)]
    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let p = self.par[x];
            let pp = self.root(p);
            self.par[x] = pp;
            pp
        }
    }
    #[allow(dead_code)]
    pub fn merge(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.par[x] = y;
        } else {
            self.par[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }
}
