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

        // if (n - s) % k == 0 {
        //     println!("{}", (n - s) / k);
        //     continue 'outer;
        // }
        // // n, k が互いに素だったら絶対に答えはあるし逆元もある(k < n)
        // if gcd(n, k) == 1 {
        //     println!("{}", ((n - s) * mod_pow(k, n - 2, n)) % n);
        //     continue 'outer;
        // }
        // use std::collections::HashSet;
        // let mut set = HashSet::new();
        // for i in 0..=n {
        //     let rem = (s + i * k) % n;
        //     if rem == 0 {
        //         println!("{}", i);
        //         continue 'outer;
        //     }
        //     if set.contains(&rem) {
        //         println!("-1");
        //         continue 'outer;
        //     }
        //     set.insert(rem);
        // }
        // for i in 1..=n {
        //     let rem = (s + i * k) % n;
        //     if
        // }
        // println!("{}", -1);

        // baby step giant step
        use std::collections::HashMap;
        let mut table = HashMap::new();
        let sqrt = (n as f64).sqrt().ceil() as u64;
        for i in 0..=sqrt {
            let pos = (k * i) % n;
            let entry = table.entry(pos).or_insert(n + 1);
            *entry = std::cmp::min(*entry, i);
        }
        for i in (0..=n).step_by(sqrt as usize) {
            let tmp = (s + k * i) % n;
            let rem = if tmp == 0 { 0 } else { n - tmp };
            match table.get(&rem) {
                None => {
                    continue;
                }
                Some(a) => {
                    println!("{}", a + i);
                    continue 'outer;
                }
            }
        }
        println!("-1");
    }
}
