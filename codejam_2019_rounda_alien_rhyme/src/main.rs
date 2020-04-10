#![allow(non_snake_case)]

#[derive(Debug)]
struct Count((String, usize));
use std::cmp::Ordering;
impl PartialEq for Count {
    fn eq(&self, other: &Self) -> bool {
        (self.0).0.len() == (other.0).0.len()
    }
}
impl Eq for Count {}
impl PartialOrd for Count {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Count {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.0).0.len().cmp(&(other.0).0.len())
    }
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
    let T: usize = read();
    for t in 1..T + 1 {
        let N: usize = read();
        use std::collections::HashMap;
        let mut map: HashMap<String, usize> = HashMap::new();
        for _ in 0..N {
            let s: String = read();
            let mut s2 = "".to_string();
            for c in s.chars().rev() {
                s2.push(c);
                let s3 = s2.clone();
                let entry = map.entry(s3).or_insert(0);
                *entry += 1;
            }
        }
        let mut ans = 0;
        let map2 = map.clone();
        use std::collections::BinaryHeap;
        let mut heap: BinaryHeap<Count> = BinaryHeap::new();
        for (key, count) in map2 {
            let count = Count((key, count));
            heap.push(count);
        }
        while let Some(count) = heap.pop() {
            dbg!(&count);
            let suffix = (count.0).0;
            let count = (count.0).1;
            if map.get(&suffix).is_none() {
                continue;
            }
            if count < 2 {
                map.remove(&suffix);
                continue;
            }
            let mut s2 = "".to_string();
            for c in suffix.chars() {
                s2.push(c);
                let s3 = s2.clone();
                let now_val;
                {
                    let entry = map.entry(s3).or_insert(0);
                    *entry -= 2;
                    now_val = *entry;
                }
                if now_val == 0 {
                    map.remove(&s2);
                }
            }
            ans += 2;
            map.remove(&suffix);
        }
        println!("Case #{}: {}", t, ans);
    }
}
