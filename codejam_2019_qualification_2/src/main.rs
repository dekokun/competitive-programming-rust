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
    let T: usize = read();
    for t in 0..T {
        let N: usize = read();
        let P: String = read();
        let vec: Vec<char> = P.chars().collect();
        // 最初と最後が同じだったらそれを避けて壁沿いに動けば良い
        if vec[0] != vec[vec.len() - 1] {
            println!(
                "Case #{}: {}{}",
                t + 1,
                vec[vec.len() - 1].to_string().repeat(N - 1),
                vec[0].to_string().repeat(N - 1)
            );
        } else {
            // 2連続してる間隙を抜く
            let search = rev(vec[0]);
            let mut before = '0';
            let mut now_index = 0;
            let mut through_index = 0;
            for &v in vec.iter() {
                if v == search {
                    now_index += 1;
                }
                if before == search && before == v {
                    through_index = now_index - 1;
                }
                before = v;
            }
            println!(
                "Case #{}: {}{}{}",
                t + 1,
                rev(vec[0]).to_string().repeat(through_index),
                vec[0].to_string().repeat(N - 1),
                rev(vec[0]).to_string().repeat(N - through_index - 1),
            );
        }
    }
}

fn rev(c: char) -> char {
    if c == 'E' {
        'S'
    } else {
        'E'
    }
}
