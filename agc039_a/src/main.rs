use std::io::*;
use std::str::FromStr;

#[allow(dead_code)]
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

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let opt = read_option();
    opt.expect("failed to parse token")
}

fn main() {
    let s: String = read();
    let k: i64 = read();
    let s2 = s.clone() + &s;
    let mut before_char = ';';
    let mut cnt = 0;
    for c in s2.chars() {
        if c == before_char {
            cnt += 1;
            before_char = ';';
        } else {
            before_char = c;
        }
    }
    let unique: std::collections::HashSet<char> = s2.chars().collect();
    // all same
    let ans = if unique.len() == 1 {
        (s.len() as i64 * k) / 2
    } else if cnt % 2 == 0 {
        (cnt + 1) / 2 * k
    } else {
        ((cnt + 1) / 2 * k) - 1
    };
    println!("{}", ans)
}
