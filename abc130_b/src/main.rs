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
    let n: u64 = read();
    let x: u64 = read();
    let mut ans: u64 = 1;
    let mut sum: u64 = 0;
    for _ in 0..n {
        sum += read();
        if sum > x {
            println!("{}", ans);
            return;
        }
        ans += 1;
    }
    println!("{}", n + 1);
}
