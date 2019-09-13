use std::collections::BTreeSet;
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
    // index: i64, value: usize
    let n: i64 = read();
    // let mut vec = vec![0; n];
    let mut p: Vec<usize> = vec![];
    let mut rev: Vec<usize> = vec![0; n as usize];
    for _ in 0..n {
        p.push(read::<usize>() - 1);
    }
    for (i, &v) in p.iter().enumerate() {
        rev[v] = i;
    }
    let mut sum: i64 = 0;
    let mut set = BTreeSet::new();
    // 上から埋めていく
    for i in (0..n).rev() {
        let index = rev[i as usize] as i64;

        set.insert(index as i64);
        // 重要。端に達した場合はそこまで数える
        let mut l = vec![-1; 2];
        let mut r = vec![n; 2];
        let mut before = set.range(..index).next_back();
        for j in 0..2 {
            if before.is_none() {
                break;
            }
            l[j] = *before.unwrap();
            before = set.range(..l[j]).next_back();
        }
        // rangeは左端は含む半開区間
        let mut after = set.range((index + 1)..).next();
        for j in 0..2 {
            if after.is_none() {
                break;
            }
            r[j] = *after.unwrap();
            after = set.range((r[j] + 1)..).next();
        }
        let mut ls = vec![0; 2];
        let mut rs = vec![0; 2];
        ls[0] = index - l[0];
        ls[1] = l[0] - l[1];
        rs[0] = r[0] - index;
        rs[1] = r[1] - r[0];
        let point = ls[0] * rs[1] + ls[1] * rs[0];
        if i == n - 1 {
            assert_eq!(0, point);
        } else {
            assert!(point >= 0, "point is negative");
        }
        sum += point * (i + 1);
    }
    println!("{}", sum);
}
