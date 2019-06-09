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
    let n: i32 = read();
    let m: i32 = read();
    let mut vec: Vec<i32> = vec![];
    let mut before = -1;
    for _ in 0..m {
        let v = read();
        if v == before + 1 {
            println!("0");
            return;
        }
        before = v;
        vec.push(v);
    }
    vec.push(n + 1);
    let mut before = 0;
    let mut ans = 1;
    for i in vec.iter() {
        let i = *i;
        let mut a1 = 0;
        let mut a2 = 1;
        let mut a3 = 1;
        for _ in before..(i - 1) {
            a3 = (a1 + a2) % 1_000_000_007;
            a1 = a2;
            a2 = a3;
        }
        ans = ((ans * a3 as u64) % 1_000_000_007) as u64;
        before = i + 1;
    }
    println!("{}", ans);
}
