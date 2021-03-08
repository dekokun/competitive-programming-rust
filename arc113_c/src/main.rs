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

fn main() {
    println!("{}", solve2(read()));
}
fn solve2(s: String) -> u64 {
    let mut before = s.chars().next().unwrap();
    // 2連続の文字、2文字目のindex, そこから最後までの間に消せない数
    let mut results = vec![];
    let mut seq = false;
    let mut count = 0;
    for (i, c) in s.chars().enumerate().skip(1) {
        if before == c {
            seq = true;
            count += 1;
            continue;
        }
        if seq {
            results.push((before, i - 1, count));
        }
        count = 0;
        seq = false;

        before = c;
    }
    let s = s.chars().collect::<Vec<_>>();
    results.push(('?', s.len(), 0));
    let mut ans = 0;
    debug!(results);
    for v in results.windows(2) {
        let (c1, i1, _count1) = v[0];
        let (c2, i2, count2) = v[1];
        // aabaac
        // 01234
        // (a, 1, 2) (a, 4, 2)
        ans += if c1 == c2 {
            (i2 - i1 - count2 - 1) as u64
        } else {
            (s.len() - i1 - 1) as u64
        };
        for &c in &s[i1 + 1..i2 - count2] {
            if c == c1 {
                ans -= 1;
            }
        }
    }
    ans
}
#[allow(dead_code)]
fn solve(s: String) -> u64 {
    let mut before = s.chars().next().unwrap();
    let mut before_double = '?';
    let mut not_delete = 0;
    // 2連続の文字、2文字目のindex, そこから最後までの間に消せない数
    let mut results = vec![];
    let mut seq = false;
    for (i, c) in s.chars().enumerate().skip(1) {
        if before == c && !seq {
            before_double = c;
            if !seq {
                results.push((c, i, not_delete));
            }
            not_delete = 0;
            seq = true;
            continue;
        }
        if before == c && seq {
            not_delete += 1;
            continue;
        }
        if before_double == c {
            not_delete += 1;
        }
        seq = false;

        before = c;
    }
    let len = s.len();
    let mut ans = 0;
    // debug!(results, not_delete);
    let last_not_delete = not_delete;
    for (_c, i, not_delete) in results {
        // debug!(len, i, not_delete, ans);
        ans += (len - i - 1) as u64;
        ans -= not_delete as u64;
    }
    ans - last_not_delete as u64
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #![allow(unused_imports)]

    use super::*;
    #[test]
    fn test1() {
        let mut rng = rand::thread_rng();
        const CHARSET: &[u8] = b"acd";
        // const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        // let len: usize = 2 * 10_usize.pow(5);
        let len: usize = 7;
        for _ in 0..100 {
            use rand::Rng;

            let password: String = (0..len)
                .map(|_| {
                    let idx = rng.gen_range(0..CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect();
            debug!(password);
            // assert_eq!(solve(password.clone()), solve2(password));
        }
    }
}
