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
        loop {
            let map2 = map.clone();
            let mut v: Vec<_> = map2.iter().collect();
            // 文字列の長さの昇順にソート
            // TODO: BinaryHeapを使用
            v.sort_by(|x, y| x.0.len().cmp(&y.0.len()));
            if let Some((suffix, &count)) = v.pop() {
                if count < 2 {
                    map.remove(suffix);
                    continue;
                }
                let mut s2 = "".to_string();
                for c in suffix.chars() {
                    s2.push(c);
                    let s3 = s2.clone();
                    let entry = map.entry(s3).or_insert(0);
                    *entry -= 2;
                }
                ans += 2;
                map.remove(suffix);
            } else {
                break;
            };
        }
        println!("Case #{}: {}", t, ans);
    }
}
