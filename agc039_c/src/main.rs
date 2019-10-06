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
    let n: u32 = read();
    let two: i64 = 2;
    eprintln!("n: {}", n);
    for i in 0..two.pow(n) {
        let mut now = i;
        let mut cnt = 0;
        // eprintln!("start {:?}", i);
        // eprintln!("{:0>1$b} start", i, n as usize);
        loop {
            cnt += 1;
            if now % 2 == 0 {
                now = (now / 2) + two.pow(n - 1);
            } else {
                now = (now - 1) / 2;
            }
            // eprintln!("{:0>1$b}", now, n as usize);
            if now == i {
                // dbg!(cnt);
                if cnt != n * 2 {
                    dbg!(i, cnt);
                }
                break;
            }
            if cnt == 100_000 {
                println!("failed");
                break;
            }
        }
    }
}
