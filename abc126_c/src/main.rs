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
    let k: i32 = read();
    let mut sum = 0_f64;
    for i in 1..(n + 1) {
        let mut j = 0;
        while i * 2_i32.pow(j) < k {
            j += 1;
        }
        sum += (1_f64 / 2_f64).powf(j as f64);
    }
    println!("{}", sum / n as f64)
}
