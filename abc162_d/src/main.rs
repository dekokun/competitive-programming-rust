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
    let _N: usize = read();
    let S: String = read();
    use std::collections::{HashMap, HashSet};
    let mut m: HashMap<char, HashSet<i32>> = HashMap::new();
    m.insert('R', HashSet::new());
    m.insert('G', HashSet::new());
    m.insert('B', HashSet::new());
    for (i, c) in S.chars().enumerate() {
        m.get_mut(&c).unwrap().insert(i as i32);
    }
    let mut ans = 0;
    for r in m.get(&'R').unwrap() {
        for g in m.get(&'G').unwrap() {
            let mut checkBs = vec![];
            if (r + g) % 2 == 0 {
                checkBs.push((r + g) / 2);
            }
            let max = std::cmp::max(r, g);
            let min = std::cmp::min(r, g);
            let low = min - (max - min);
            let high = max + (max - min);
            checkBs.push(low);
            checkBs.push(high);
            let bs = m.get(&'B').unwrap();
            let mut tmp = bs.len();
            for v in checkBs {
                if bs.get(&v).is_some() {
                    tmp -= 1;
                }
            }
            ans += tmp;
        }
    }
    println!("{}", ans);
}
