use std::collections::HashSet;
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

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    if b == 0 {
        return a;
    }
    let mut r;
    while (a % b) != 0 {
        r = a % b;
        a = b;
        b = r;
    }
    b
}
#[test]
fn test_gcd() {
    assert_eq!(gcd(10, 5), 5);
    assert_eq!(gcd(20, 30), 10);
    assert_eq!(gcd(100, 3), 1);
    assert_eq!(gcd(3, 60), 3);
    assert_eq!(gcd(30, 20), 10);
}

fn main() {
    let a: i64 = read();
    let b: i64 = read();
    let gcd = gcd(a, b);
    let mut set: HashSet<i64> = HashSet::new();
    let sqrt = (gcd as f64).sqrt().ceil() as usize;
    let mut now = gcd;
    for i in 2..sqrt + 100 {
        let i = i as i64;
        while now % i == 0 {
            set.insert(i);
            now = now / i;
        }
        if now == 1 {
            break;
        }
    }
    if now != 1 {
        set.insert(now);
    }
    println!("{}", set.len() + 1);
}
