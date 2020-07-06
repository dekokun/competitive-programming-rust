#![allow(non_snake_case)]
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
    let N: usize = read();
    let K: usize = read();
    let mut vec: Vec<i64> = (0..N).map(|_| read()).collect();
    let MOD = 10_i64.pow(9) + 7;
    vec.sort();
    if vec.iter().all(|&x| x < 0) && K % 2 == 1 {
        vec.reverse();
        let mut tmp: i64 = 1;
        for &i in &vec[0..K] {
            tmp *= i as i64;
            tmp %= MOD;
        }
        println!("{}", tmp + MOD);
        return;
    }
    let mut negatives: Vec<i64> = vec
        .iter()
        .filter(|&x| x < &0)
        .copied()
        .map(|x| -x)
        .collect();
    let mut positives: Vec<i64> = vec.iter().filter(|&x| x >= &0).copied().collect();
    negatives.sort();
    positives.sort();
    let mut ans_vec: Vec<i64> = vec![];
    while ans_vec.len() != K {
        if positives.is_empty() {
            let v = negatives.pop().unwrap();
            ans_vec.push(-v);
            continue;
        }
        if negatives.len() < 2 {
            let v = positives.pop().unwrap();
            ans_vec.push(v);
            continue;
        }
        if K - ans_vec.len() == 1 {
            let v = positives.pop().unwrap();
            ans_vec.push(v);
            continue;
        }
        if positives.len() == 1 && K >= 2 {
            let v1 = negatives.pop().unwrap();
            let v2 = negatives.pop().unwrap();
            ans_vec.push(-v1);
            ans_vec.push(-v2);
            continue;
        }
        if positives.len() == 1 && K == 1 {
            let v = positives.pop().unwrap();
            ans_vec.push(v);
            continue;
        }
        let neg = negatives[negatives.len() - 1] * negatives[negatives.len() - 2];
        let pos = positives[positives.len() - 1] * positives[positives.len() - 2];
        if pos > neg {
            let v = positives.pop().unwrap();
            ans_vec.push(v);
        } else {
            let v1 = negatives.pop().unwrap();
            let v2 = negatives.pop().unwrap();
            ans_vec.push(-v1);
            ans_vec.push(-v2);
        }
    }
    let mut ans: i64 = 1;
    for v in ans_vec {
        ans *= v;
        ans %= MOD;
    }
    println!("{}", ans)
}
