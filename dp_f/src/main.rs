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
    let s: String = read();
    let t: String = read();
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();
    let mut dp: Vec<Vec<(usize, String)>> =
        vec![vec![(0, "".to_string()); t.len() + 1]; s.len() + 1];
    for i in 0..s.len() {
        for j in 0..t.len() {
            let mut vec = if s[i] == t[j] {
                let mut s2 = dp[i][j].1.clone();
                s2.push(s[i]);
                vec![
                    (dp[i][j].0 + 1, s2),
                    dp[i][j + 1].clone(),
                    dp[i + 1][j].clone(),
                ]
            } else {
                vec![
                    (dp[i][j].0, dp[i][j].1.clone()),
                    dp[i][j + 1].clone(),
                    dp[i + 1][j].clone(),
                ]
            };
            vec.sort_by_key(|v| v.0);
            dp[i + 1][j + 1] = vec.pop().unwrap();
        }
    }
    // dbg!(&dp[s.len()][t.len()]);
    println!("{}", dp[s.len()][t.len()].1);
}
