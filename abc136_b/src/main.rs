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
    let n: i32 = read::<String>().parse().unwrap();
    let mut ans = 0;
    for i in 1..(n + 1) {
        let s = i.to_string();
        if s.len() % 2 == 1 {
            // dbg!(s.len(), s, i);
            ans += 1;
        }
    }
    println!("{}", ans);
}
