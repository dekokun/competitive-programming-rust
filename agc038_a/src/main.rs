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
    let h: usize = read();
    let w: usize = read();
    let a: usize = read();
    let b: usize = read();
    let mut vec = vec![vec![0; w]; h];
    if a == 0 {
        for i in 0..b {
            let row = &mut vec[i];
            for j in 0..w {
                row[j] = 1;
            }
        }
    }
    if b == 0 {
        for row in vec.iter_mut() {
            for i in 0..a {
                row[i] = 1;
            }
        }
    } else {
        for row in vec.iter_mut() {
            for i in 0..a {
                row[i] = 1;
            }
        }
        let mut plus_column = a;
        let mut count = 0;
        for i in b..h {
            for j in 0..a {
                if count == b {
                    plus_column += a;
                }
                if j + plus_column >= w {
                    continue;
                }
                vec[i].swap(j, j + plus_column);
                count += 1;
            }
        }
    }
    // a check
    for row in vec.iter() {
        let mut a_count = 0;
        for v in row.iter() {
            if *v == 1 {
                a_count += 1;
            }
        }
        if std::cmp::min(a_count, w - a_count) != a {
            println!("-1");
            return;
        }
    }
    // b check
    let mut b_counts = vec![0; w];
    for row in vec.iter() {
        for (i, v) in row.iter().enumerate() {
            if *v == 1 {
                b_counts[i] += 1;
            }
        }
    }
    for v in b_counts {
        if std::cmp::min(v, h - v) != b {
            println!("-1");
            return;
        }
    }

    for row in vec.iter() {
        println!(
            "{}",
            row.iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}
