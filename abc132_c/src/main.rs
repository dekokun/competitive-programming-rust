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
    let n = read();
    let mut vec: Vec<i32> = vec![];
    for _ in 0..n {
        vec.push(read());
    }
    vec.sort();
    let l = vec[vec.len() / 2 - 1];
    let r = vec[vec.len() / 2];
    if l == r {
        println!("0");
        return;
    }
    println!("{}", r - l);
}
