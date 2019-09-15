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
    let n: usize = read();
    let k: usize = read();
    let q: usize = read();
    let mut user_points: Vec<i32> = vec![-(q as i32); n];
    for _ in 0..q {
        user_points[read::<usize>() - 1] += 1;
    }
    for i in user_points {
        if i > -(k as i32) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
