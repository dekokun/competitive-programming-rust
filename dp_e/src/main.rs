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
    let w: usize = read();
    const INF: i64 = std::i64::MAX / 3;
    let v_max: usize = 1000 * n + 1;
    // (weight, value)
    let mut vec: Vec<(i64, i64)> = vec![];
    for _ in 0..n {
        vec.push((read(), read()));
    }
    let mut dp: Vec<Vec<i64>> = vec![vec![INF; v_max + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..v_max {
            if j < vec[i].1 as usize {
                dp[i + 1][j] = dp[i][j];
                continue;
            }
            dp[i + 1][j] = std::cmp::min(dp[i][j - vec[i].1 as usize] + vec[i].0, dp[i][j])
        }
    }
    let mut max_val = 0;
    for v1 in dp {
        for (i, &v2) in v1.iter().enumerate() {
            if v2 <= w as i64 {
                max_val = std::cmp::max(max_val, i);
            }
        }
    }

    println!("{}", max_val);
}
