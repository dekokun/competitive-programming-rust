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
    let n: i64 = read();
    let k: i64 = read();
    let mut vec: Vec<i64> = vec![];
    let mut ans = 0;
    let mut l = 0;
    let mut r = 0;
    let mut sum: i64 = 0;
    for _ in 0..n {
        vec.push(read());
    }
    loop {
        // eprintln!("sum: {}, l: {}, r: {}, ans: {}, k: {}", sum, l, r, ans, k);
        if l == n {
            break;
        }
        if sum >= k {
            ans += n - r + 1;
            sum -= vec[l as usize];
            l += 1;
        } else if r == n {
            break;
        } else {
            sum += vec[r as usize];
            r += 1;
        }
    }
    println!("{}", ans);
}
