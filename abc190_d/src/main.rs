#![allow(non_snake_case, unused_macros)]

// https://maguro.dev/debug-macro/ $B$+$i(B
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

use std::io::{stdin, Read};
use std::str::FromStr;
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
fn read<T: FromStr>() -> T {
    let opt = read_option();
    opt.expect("failed to parse token")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, true);
    }
}

fn main() {
    let n = read();
    println!("{}", solve(n));
}

fn solve(n: u64) -> u64 {
    // 1以上から構成されるものを計算し、それを2倍すれば良い
    let mut ans = 0;
    debug!((n as f64).sqrt().ceil());
    // for i in 1..=n {
    for i in 1..=100 * (n as f64).sqrt().ceil() as u64 {
        // 奇数は、その数でnを割り切れればOK
        // 偶数は、iの半分で割り切れた数を2個の連続した数で構成できればok
        // 2個の連続した数で構成できるとは、奇数であること
        // どちらも0に突っ込む場合はすでに計算済みなので計算しない
        if i % 2 == 1 {
            if n % i == 0 && n / i > i / 2 {
                debug!(i);
                ans += 1;
            }
        } else if n % (i / 2) == 0 {
            if (n / (i / 2)) % 2 == 1 && (n / (i / 2)) / 2 >= i / 2 {
                debug!(i);
                ans += 1;
            }
        }
    }
    ans * 2
}
