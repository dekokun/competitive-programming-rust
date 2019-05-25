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
    let id_count: i64 = read();
    let gate_count: i64 = read();
    let mut max = id_count;
    let mut min = 1;
    for _ in 0..gate_count {
        let l = read();
        if l > min {
            min = l;
        }
        let r = read();
        if r < max {
            max = r;
        }
    }
    if max < min {
        println!("0");
    } else {
        println!("{}", max - min + 1);
    }
}

