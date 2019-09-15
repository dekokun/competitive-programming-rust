use std::collections::BinaryHeap;
use std::io::*;
use std::str::FromStr;

#[allow(dead_code)]
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

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let opt = read_option();
    opt.expect("failed to parse token")
}

fn main() {
    let mut queue = BinaryHeap::new();
    let n: i32 = read();
    let m: i64 = read();
    for _ in 0..n {
        let price: i64 = read();
        queue.push(price);
    }
    for _ in 0..m {
        let price = queue.pop().unwrap();
        // round
        queue.push(price / 2);
    }
    let mut sum = 0;
    for price in queue {
        sum += price;
    }
    println!("{}", sum);
}
