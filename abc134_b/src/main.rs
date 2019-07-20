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
    let d: i32 = read();
    let one_tree = 2 * d + 1;
    let quotient = n / one_tree;
    let reminder = n % one_tree;
    let ans = if reminder == 0{
        quotient
    } else {
        quotient + 1
    };
    println!("{}", ans);
}
