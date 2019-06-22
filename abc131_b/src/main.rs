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
    let n: i16 = read();
    let l: i16 = read();
    let eaten;
    if l - 1 < - n  {
        eaten = l + n - 1;
    } else if l > 0 {
        eaten = l + 1 - 1;
    } else {
        eaten = 0;
    }
    let mut sum = 0;
    for i in 1..n + 1 {
        sum += l + i - 1;
    }
    println!("{}", sum - eaten);
}
