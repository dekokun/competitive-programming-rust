use std::collections::BinaryHeap;
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
    let n: usize = read();
    let k: usize = read();
    let mut queue: Vec<i32> = vec![];
    for _ in 0..n {
        queue.push(read());
    }
    let mut ans = 0;
    for l in 0..(std::cmp::min(n, k) + 1) {
        for r in 0..(std::cmp::min(n, k) - l + 1) {
            let mut now = 0;
            let mut hands: BinaryHeap<i32> = BinaryHeap::new();
            for i in 0..l {
                now += queue[i];
                hands.push(-queue[i]);
            }
            for j in 0..r {
                now += queue[n - j - 1];
                hands.push(-queue[n - j - 1]);
            }
            for _ in 0..k - (r + l) {
                let minus_value = match hands.pop() {
                    Some(i) => i,
                    None => break,
                };
                if minus_value < 0 {
                    break;
                }
                // negate value
                now += minus_value;
            }
            ans = std::cmp::max(ans, now);
        }
    }
    println!("{}", ans);
}
