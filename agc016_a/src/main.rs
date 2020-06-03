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
    // 各文字の出現位置を取得
    // 最後の出現が、文字列の最後からどれくらいか
    // 各出現の間隔はどれくらいか、の最大値を使用する
    // -1と最後 + 1にその文字があると仮定して計算すると楽か
    let s: String = read();
    use std::collections::HashMap;
    let mut map: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        let entry = map.entry(c).or_insert_with(|| vec![s.len()]);
        entry.push(i);
    }
    let mut ans = std::usize::MAX;
    use std::cmp::{max, min};
    for v in map.values_mut() {
        v.sort();
        let mut ans_now = v[0];
        for vec in v.windows(2) {
            let a = vec[0];
            let b = vec[1];
            ans_now = max(ans_now, b - a - 1);
        }
        ans = min(ans, ans_now);
    }
    println!("{}", ans);
}
