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

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
fn lcm(a: u64, b: u64) -> u64 {
    let gcd = gcd(a, b);
    (a / gcd) * b
}

fn main() {
    let a: u64 = read();
    let b: u64 = read();
    let c: u64 = read();
    let d: u64 = read();
    let all_count = b - a + 1;
    let c_count = (b / c) - ((a - 1) / c);
    let d_count = (b / d) - ((a - 1) / d);

    let lcm = lcm(c, d);
    let lcm_count = (b / lcm) - ((a -1)/ lcm);
    println!(
        "{}",
        std::cmp::max(
            all_count as i64 - c_count as i64 - d_count as i64 + lcm_count as i64,
            0
        )
    );
}
