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
    // let mut vec = vec![0; n];
    let mut p: Vec<usize> = vec![];
    let mut sum = 0;
    let mut max;
    let mut second;
    for _ in 0..n {
        p.push(read());
    }
    for i in 0..n {
        for j in i + 1..n {
            max = 0;
            second = 0;
            for k in i..j + 1 {
                // vec[k] += 1;
                if max < p[k] {
                    second = max;
                    max = p[k];
                } else if second < p[k] {
                    second = p[k];
                }
            }
            sum += second;
        }
    }
    println!("{}", sum);
}
