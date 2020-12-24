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
    let t: usize = read();
    'outer: for _ in 0..t {
        let n: u64 = read();
        let s: u64 = read();
        let k: u64 = read();

        // baby step giant step
        use std::collections::HashMap;
        let mut table = HashMap::new();
        let sqrt = (n as f64).sqrt().ceil() as u64;
        for i in 0..=sqrt {
            let pos = (k * i) % n;
            if ! table.contains_key(&pos) {
                table.insert(pos, i);
            }
        }
        for i in (0..=n).step_by(sqrt as usize) {
            let tmp = (s + k * i) % n;
            let rem = (n - tmp) % n;
            if let Some(a) = table.get(&rem) {
                println!("{}", a + i);
                continue 'outer;
            }
        }
        println!("-1");
    }
}
