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
    let s: String = read();
    // 0: remain: 0
    let mut dp: Vec<[usize; 13]> = vec![[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
    for (i, c) in s.chars().rev().enumerate() {
        let arr = [0; 13];
        dp.push(arr);
        if c == '?' {
            for n in 0..10 {
                let remain = (n * pow_modulo13(10, i)) % 13;
                for (j, v) in dp[i].clone().iter().enumerate() {
                    dp[i + 1][(j + remain) % 13] += *v;
                    dp[i + 1][(j + remain) % 13] %= 10_usize.pow(9) + 7;
                }
            }
        } else {
            let n = c.to_digit(10).unwrap() as usize;
            let remain = (n * pow_modulo13(10, i)) % 13;
            for (j, v) in dp[i].clone().iter().enumerate() {
                dp[i + 1][(j + remain) % 13] += *v;
                dp[i + 1][(j + remain) % 13] %= 10_usize.pow(9) + 7;
            }
        }
    }
    println!("{}", dp[dp.len() - 1][5]);
}
fn pow_modulo13(n: usize, exp: usize) -> usize {
    let mut ans = 1;
    for _ in 0..exp {
        ans *= n;
        ans %= 13;
    }
    ans
}
