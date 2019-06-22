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
    let mut vec = vec![];
    for _ in 0..n {
        let a: u64 = read();
        let b: u64 = read();
        vec.push((a, b));
    }
    vec.sort_by_key(|k| k.1);
    let mut now = 0;
    for i in vec {
        let time = i.0;
        let shime = i.1;
        now += time;
        if (now > shime) {
            println!("No");
            return;
        }
    }
            println!("Yes");
}

