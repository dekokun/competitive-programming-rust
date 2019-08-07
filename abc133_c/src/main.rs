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
    let l: i64 = read();
    let r: i64 = read();
    if r - l > 2019 {
        println!("{}", 0);
        return;
    }
    let mut ans = 2019;
    for i in l..r {
        for j in (i + 1)..(r + 1) {
            let v = ((i % 2019) * (j % 2019)) % 2019;
            ans = std::cmp::min(v, ans);
        }
    }
    println!("{}", ans);
}
