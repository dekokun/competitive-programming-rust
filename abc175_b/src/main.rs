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
    let N: usize = read();
    let l: Vec<usize> = (0..N).map(|_| read()).collect();
    use std::collections::HashMap;
    let mut map: HashMap<usize, usize> = HashMap::new();
    for v in l {
        let entry = map.entry(v).or_insert(0);
        *entry += 1;
    }
    let mut ans = 0;
    for &i in map.keys() {
        for &j in map.keys() {
            if i >= j {
                continue;
            }
            for &k in map.keys() {
                if j >= k {
                    continue;
                }
                // i < j < k
                if i + j > k {
                    ans += map.get(&i).unwrap() * map.get(&j).unwrap() * map.get(&k).unwrap()
                }
            }
        }
    }
    println!("{}", ans);
}
