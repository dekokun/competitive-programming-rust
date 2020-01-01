use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let n: i32 = read();
    let m: i32 = read();
    let mut jobs: HashMap<i32, Vec<i32>> = HashMap::new();
    for _ in 0..n {
        let day: i32 = read();
        let reward: i32 = read();
        jobs.entry(day).or_insert(vec![]).push(reward);
    }
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut sum = 0;
    for i in (0..m).rev() {
        for v in jobs.entry(m - i).or_insert(vec![]) {
            heap.push(*v);
        }
        match heap.pop() {
            Some(i) => {
                sum += i;
            }
            None => {}
        }
    }
    println!("{}", sum);
}
