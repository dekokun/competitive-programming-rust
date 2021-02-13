#![allow(non_snake_case, unused_macros)]

// https://maguro.dev/debug-macro/ $B$+$i(B
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

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
    let h: usize = read();
    let w: usize = read();
    let mut link = vec![];
    for _ in 0..h {
        link.push(read::<String>().chars().map(|c| c == '#').collect())
    }

    println!("{}", solve(h, w, link));
}

// linkはtrueの場合、壁
fn solve(h: usize, w: usize, mut link: Vec<Vec<bool>>) -> usize {
    // 壁を全部つなぐのに何個？
    //   ふちにあれば繋がってる
    //   繋がってないやつ同士のグループの数だ！
    // 壁のないところを埋めるのに何個？
    //   縦と横で壁がない列の少ない方の壁の個数さえあればOK
    link[0][0] = true;
    link[h - 1][w] = true;
    let wall_count = link
        .iter()
        .map(|row| row.iter().filter(|a| **a).count())
        .sum();
    debug!(wall_count);
    let mut dsu = Dsu::new(wall_count);
    let mut count = 0;
    let mut rows = vec![vec![]; h];
    let mut columns = vec![vec![]; w];
    let mut edge = false;
    for (i, row) in link.into_iter().enumerate() {
        for (j, cell) in row.into_iter().enumerate() {
            if cell {
                for &r in &rows[i] {
                    dsu.merge(count, r);
                }
                for &c in &columns[j] {
                    dsu.merge(count, c);
                }
                if i == 0 || i == h - 1 || j == 0 || j == w - 1 {
                    edge = true;
                }
                rows[i].push(count);
                columns[j].push(count);
                count += 1;
            }
        }
    }

    let wall_connect_count = dsu.groups().len() - if edge { 1 } else { 0 };
    let mut rows_connect_count = 0;
    for i in 0..h {
        if i == 0 || i == h - 1 {
            continue;
        }
        if rows[i].is_empty() {
            rows_connect_count += 1;
        }
    }
    let mut columns_connect_count = 0;
    for i in 0..w {
        if i == 0 || i == w - 1 {
            continue;
        }
        if columns[i].is_empty() {
            columns_connect_count += 1;
        }
    }
    debug!(
        wall_connect_count,
        rows_connect_count, columns_connect_count
    );
    wall_connect_count + std::cmp::min(rows_connect_count, columns_connect_count)
}

//https://github.com/rust-lang-ja/ac-library-rs

pub mod dsu {
    /// Implement (union by size) + (path compression)
    /// Reference:
    /// Zvi Galil and Giuseppe F. Italiano,
    /// Data structures and algorithms for disjoint set union problems
    pub struct Dsu {
        n: usize,
        // root node: -1 * component size
        // otherwise: parent
        parent_or_size: Vec<i32>,
    }

    impl Dsu {
        // 0 <= size <= 10^8 is constrained.
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                parent_or_size: vec![-1; size],
            }
        }
        pub fn merge(&mut self, a: usize, b: usize) -> usize {
            assert!(a < self.n);
            assert!(b < self.n);
            let (mut x, mut y) = (self.leader(a), self.leader(b));
            if x == y {
                return x;
            }
            if -self.parent_or_size[x] < -self.parent_or_size[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.parent_or_size[x] += self.parent_or_size[y];
            self.parent_or_size[y] = x as i32;
            x
        }

        pub fn same(&mut self, a: usize, b: usize) -> bool {
            assert!(a < self.n);
            assert!(b < self.n);
            self.leader(a) == self.leader(b)
        }
        pub fn leader(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            if self.parent_or_size[a] < 0 {
                return a;
            }
            self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
            self.parent_or_size[a] as usize
        }
        pub fn size(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            let x = self.leader(a);
            -self.parent_or_size[x] as usize
        }
        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            let mut leader_buf = vec![0; self.n];
            let mut group_size = vec![0; self.n];
            for i in 0..self.n {
                leader_buf[i] = self.leader(i);
                group_size[leader_buf[i]] += 1;
            }
            let mut result = vec![Vec::new(); self.n];
            for i in 0..self.n {
                result[i].reserve(group_size[i]);
            }
            for i in 0..self.n {
                result[leader_buf[i]].push(i);
            }
            result
                .into_iter()
                .filter(|x| !x.is_empty())
                .collect::<Vec<Vec<usize>>>()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn dsu_works() {
            let mut d = Dsu::new(4);
            d.merge(0, 1);
            assert_eq!(d.same(0, 1), true);
            d.merge(1, 2);
            assert_eq!(d.same(0, 2), true);
            assert_eq!(d.size(0), 3);
            assert_eq!(d.same(0, 3), false);
            assert_eq!(d.groups(), vec![vec![0, 1, 2], vec![3]]);
        }
    }
}
use dsu::*;
