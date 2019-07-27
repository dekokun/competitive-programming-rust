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
    let mut cnt: i32 = 0;
    let mut another_count = 0;
    let n = read();
    for _ in 0..n {
        cnt += 1;
        let now: i32 = read();
        if now != cnt {
            if another_count == 0 {
                another_count = now;
                continue;
            }
            if another_count != cnt {
                println!("NO");
                return;
            }
            continue;
        }
    }
    println!("YES");
}
