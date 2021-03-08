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
    println!(
        "{}",
        solve(
            (0..10)
                .map(|_| read::<String>().chars().map(|c| c == 'o').collect())
                .collect()
        )
    );
}

// map: 陸地はtrue
fn solve(map: Vec<Vec<bool>>) -> String {
    debug!(map);
    let land_count: usize = map
        .iter()
        .map(|row| row.iter().filter(|cell| **cell).count())
        .sum();
    let mut dsu = Dsu::new(land_count + 1);
    let mut map_index = vec![vec![None; 10]; 10];
    let mut index = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell {
                map_index[i][j] = Some(index);
                index += 1;
            }
        }
    }
    for i in 0..10 {
        for j in 0..10 {
            if map_index[i][j].is_none() {
                continue;
            }
            for &(add_i, add_j) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let i = i as i32;
                let j = j as i32;
                if add_i + i < 0 || add_j + j < 0 || add_i + i >= 10 || add_j + j >= 10 {
                    continue;
                }
                match map_index[(add_i + i) as usize][(add_j + j) as usize] {
                    None => continue,
                    Some(v) => {dsu.merge(v, map_index[i as usize][j as usize].unwrap())}
                };
            }
        }
    }
    for i in 0..10 {
        for j in 0..10 {
            let mut dsu = dsu.clone();
            let mut map_index = map_index.clone();
            map_index[i][j] = Some(land_count);
            for &(add_i, add_j) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let i = i as i32;
                let j = j as i32;
                if add_i + i < 0 || add_j + j < 0 || add_i + i >= 10 || add_j + j >= 10 {
                    continue;
                }
                match map_index[(add_i + i) as usize][(add_j + j) as usize] {
                    None => continue,
                    Some(v) => {dsu.merge(v, map_index[i as usize][j as usize].unwrap())}
                };
            }
            if dsu.groups().len() == 1 {
                return "YES".into()
            }
        }
    }
    "NO".into()
}

//https://github.com/rust-lang-ja/ac-library-rs

pub mod dsu {
    /// Implement (union by size) + (path compression)
    /// Reference:
    /// Zvi Galil and Giuseppe F. Italiano,
    /// Data structures and algorithms for disjoint set union problems
    #[derive(Clone)]
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
