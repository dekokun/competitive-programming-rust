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
    let M: usize = read();
    let max = 100;
    let mut vec: Vec<usize> = vec![max; N];
    for _ in 0..M {
        let s: usize = read();
        let s = s - 1;
        let c: usize = read();
        // 矛盾した指示
        if vec[s] != max && vec[s] != c {
            println!("-1");
            return;
        }
        // 一番左が0
        if N >= 2 && s == 0 && c == 0 {
            println!("-1");
            return;
        }
        vec[s] = c;
    }
    let mut ans = 0;
    for i in 0..N {
        // 指定がない場合
        ans += (if vec[i] == max {
            // 一番左(0以外)
            if i == 0 && N >= 2 {
                1
            } else {
                0
            }
        } else {
            vec[i]
        }) * 10_usize.pow((N - i - 1) as u32)
    }
    println!("{}", ans);
}
