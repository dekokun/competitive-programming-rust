use std::io::{stdin, Read};
use std::str::FromStr;
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
fn read<T: FromStr>() -> T {
    let opt = read_option();
    opt.expect("failed to parse token")
}

fn main() {
    let n: Vec<_> = read::<String>().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let k: usize = read();
    // 桁DP
    // [左からの桁数][smaller(0 or 1)][0じゃない桁の数]
    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; k + 1]; 2]; n.len() + 1];
    for i in 0..=n.len() {
        for smaller in 0..=1 {
            for j in 0..=k {
                for digit in 0..=9 {
                    if smaller == 0 &&
                }
            }
        }
    }
}
