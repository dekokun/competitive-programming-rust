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
    let mut vec: Vec<u64> = vec![];;
    for _ in 0..n {
        vec.push(read());
    }
    vec.reverse();
    let mut before = 0;
    for (i, v) in vec.into_iter().enumerate() {
        if i == 0 {
            before = v;
            continue;
        }
        if before >= v {
            before = v;
            continue;
        } else if before < v - 1 {
            println!("No");
            return;
        } else {
            before = v - 1;
        }
    }
    println!("Yes");
    return;
}
