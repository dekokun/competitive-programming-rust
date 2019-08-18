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
    let n: i8 = read();
    let mut vec = vec![];
    for _ in 0..n {
        let v: f64 = read();
        vec.push(v);
    }
    //reverse order
    vec.sort_by(|a, b| b.partial_cmp(a).unwrap());
    for _ in 0..(n - 1) {
        let v1 = vec.pop().unwrap();
        let v2 = vec.pop().unwrap();
        vec.push((v1 + v2) / 2.0);
        vec.sort_by(|a, b| b.partial_cmp(a).unwrap());
    }
    println!("{}", vec.pop().unwrap());
}
