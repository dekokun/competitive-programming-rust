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
    let n: i32 = read();
    let mut vec: Vec<i32> = vec![];
    for _ in 0..n {
        vec.push(read());
    }
    let mut before = 0;
    let mut ans = 0;
    let mut now = 0;
    for v in vec {
        if before >= v {
            now += 1;
            ans = std::cmp::max(ans, now);
        } else {
            ans = std::cmp::max(ans, now);
            now = 0;
        }
        before = v;
    }
    println!("{}", ans);
}
