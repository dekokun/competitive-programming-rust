use std::io::*;
use std::str::FromStr;

// not passed

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
    let n: i64 = read();
    let mut a_s: Vec<i64> = vec![];
    for _ in 0..n {
        a_s.push(read());
    }
    for (i, x) in a_s.iter().enumerate() {
        let (before, after) = if i == 0 {
            (a_s[a_s.len() - 1], a_s[i + 1])
        } else if i == a_s.len() - 1 {
            (a_s[i - 1], a_s[0])
        } else {
            (a_s[i - 1], a_s[i + 1])
        };
        if (before ^ after) != *x {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
