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
    let n: u32 = read();
    let mut vec: Vec<i32> = vec![];
    for _ in 0..n {
        vec.push(read());
    }
    let all_sum: i32 = vec.iter().fold(0, |sum, a| sum + a);
    let middle = all_sum / 2;
    let mut sum: i32 = 0;
    for v in vec.iter() {
        sum += *v;
        if sum >= middle {
            if (sum - middle) < (middle + *v - sum) {
                println!("{}", (all_sum - 2 * sum).abs());
            } else {
                println!("{}", (all_sum - 2 * (sum - *v)).abs());
            }
            return;
        }
    }
}
