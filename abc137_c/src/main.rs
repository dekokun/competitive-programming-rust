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
    let n: i32 = read();
    let mut set = HashMap::new();
    for _ in 0..n {
        let str: String = read();
        let mut str = str.chars().collect::<Vec<_>>();
        str.sort();
        *set.entry(str).or_insert(0) += 1;
    }
    let mut sum = 0;
    for (_, num) in set {
        sum += comb_2(num);
    }
    println!("{}", sum);
}

fn comb_2(n: i32) -> i64 {
    let n: i64 = n as i64;
    n * (n - 1) / 2
}
