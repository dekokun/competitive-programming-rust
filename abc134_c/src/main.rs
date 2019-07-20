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
    let mut top = 0;
    let mut second = 0;
    let mut vec: Vec<i32> = vec![];
    for _ in 0..n {
        let v = read();
        vec.push(v);
        if v > top {
            second = top;
            top = v;
        } else if v > second {
            second = v;
        }
    }
    for v in vec {
        if v == top {
            println!("{}", second);
        } else {
            println!("{}", top);
        }
    }
}
