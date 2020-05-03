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
    let T: usize = read();
    for t in 1..T + 1 {
        let max: i64 = 10_i64.pow(read::<u32>()) - 1;
        let mut vec: Vec<String> = vec![];
        for _ in 0..10_usize.pow(4) {
            let _ = read::<i64>();
            vec.push(read());
        }
        let ans = solve(max, vec);
        println!("Case #{}: {}", t, ans);
    }
}

fn solve(_max: i64, vec: Vec<String>) -> String {
    use std::collections::HashMap;
    let mut map: HashMap<char, usize> = HashMap::new();
    for s in vec {
        let mut chars = s.chars();
        let c = chars.next().unwrap();
        {
            let entry = map.entry(c).or_insert(0);
            *entry += 1;
        }
        for c in chars {
            map.entry(c).or_insert(0);
        }
    }
    use std::collections::BTreeMap;
    let mut bt: BTreeMap<usize, char> = BTreeMap::new();
    for (k, v) in map {
        bt.insert(v, k);
    }
    let mut ans_vec = vec![];
    for (_k, v) in bt {
        ans_vec.push(v);
    }
    let zero = ans_vec.remove(0);
    ans_vec.push(zero);
    ans_vec.reverse();
    ans_vec.into_iter().collect()
}
