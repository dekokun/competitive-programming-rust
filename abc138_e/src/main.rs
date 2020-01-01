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
    let s: String = read();
    let t: String = read();
    let mut map = HashMap::new();
    for (i, c) in (s.clone() + &s).chars().enumerate() {
        let entry = map.entry(c).or_insert(vec![]);
        entry.push(i);
    }
    let n = s.len();
    let mut ans = 0;
    let mut now = 0;
    for c in t.chars() {
        match map.get(&c) {
            Some(vec) => {
                match vec.binary_search(&now) {
                    Ok(idx) => {
                        now = vec[idx] + 1;
                    }
                    Err(idx) => {
                        now = vec[idx] + 1;
                    }
                }
                if now >= n {
                    now -= n;
                    ans += n;
                }
            }
            None => {
                println!("-1");
                return;
            }
        }
    }
    ans += now;
    println!("{}", ans);
}
