#![allow(non_snake_case)]

use std::cmp::min;
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
    let T: usize = read();
    for _ in 0..T {
        let (N, A, B, C, D): (i64, i64, i64, i64, i64) = (read(), read(), read(), read(), read());
        let ans = solve(N, A, B, C, D);
        println!("{}", ans);
    }
}

#[allow(dead_code)]
fn solve(N: i64, A: i64, B: i64, C: i64, D: i64) -> i64 {
    use std::collections::{HashMap, VecDeque};
    let mut dp = HashMap::new();
    let mut new = VecDeque::new();
    dp.insert(N, 0);
    new.push_front((N, 0));
    let mut min_cost: i64 = N.saturating_mul(D);
    while let Some((v, now_cost)) = new.pop_back() {
        if v == 0 {
            min_cost = min(min_cost, now_cost);
            continue;
        }
        if now_cost >= min_cost {
            continue;
        }
        for &(d, cost) in &[(2, A), (3, B), (5, C)] {
            let nexts = match v % d {
                0 => vec![(v / d, 0)],
                rem => vec![
                    ((v - rem) / d, rem * D),
                    ((v + (d - rem)) / d, (d - rem) * D),
                ],
            };
            for (next, add_cost) in nexts {
                // 後者は全部1を引いた場合と比較している。全部1が少ないコストの場合はadd_costなしで最初からv - nextに直線で行けば良い
                let new_cost = min(
                    now_cost + cost + add_cost,
                    now_cost.saturating_add((v - next).saturating_mul(D)),
                );

                match dp.get(&next) {
                    None => {
                        dp.insert(next, new_cost);
                        new.push_front((next, new_cost))
                    }
                    Some(&cost2) => {
                        // 新しい道のほうが安い場合はコスト更新するためにキューに突っ込む
                        if cost2 > new_cost {
                            dp.insert(next, new_cost);
                            new.push_front((next, new_cost))
                        }
                    }
                }
            }
        }
    }
    min_cost
}
