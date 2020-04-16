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

use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Rev<T>(pub T);

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}

fn main() {
    let N: usize = read();
    let vec = (0..N).map(|_| read::<usize>()).collect::<Vec<usize>>();
    if vec[0] != 0 {
        println!("-1");
        return;
    }
    for v in vec.windows(2) {
        let before = v[0];
        let now = v[1];
        if now > before + 1 {
            println!("-1");
            return;
        }
    }
    let mut ans = 0;
    use std::collections::{BinaryHeap, HashSet};
    let mut set = HashSet::new();
    let mut heap = BinaryHeap::new();

    for (minus_val, &v) in vec.iter().rev().enumerate() {
        if v == 0 {
            set.clear();
            heap.clear();
            continue;
        }
        if let Some(Rev(v)) = heap.peek() {
            if (v - minus_val) == 0 {
                set.remove(v);
                heap.pop();
            }
        }
        if !set.contains(&(v + minus_val)) {
            set.insert(v + minus_val);
            heap.push(Rev(v + minus_val));
        }
        ans += set.len();
    }
    println!("{}", ans);
}
