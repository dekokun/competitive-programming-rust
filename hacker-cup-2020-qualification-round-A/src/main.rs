#![allow(non_snake_case)]

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
    let T: usize = read();
    for t in 1..=T {
        let N: usize = read();
        let I: Vec<bool> = read::<String>().chars().map(|c| c == 'Y').collect();
        let O: Vec<bool> = read::<String>().chars().map(|c| c == 'Y').collect();
        let mut ans = vec![];
        for i in 0..N {
            let mut no_flg = !O[i];
            let mut tmp = vec!['Y'; N];
            for j in (0..i).rev() {
                if no_flg {
                    tmp[j] = 'N';
                    continue;
                }
                if !I[j] {
                    tmp[j] = 'N';
                    no_flg = true;
                }
                if !O[j] {
                    no_flg = true;
                }
            }
            let mut no_flg = !O[i];
            for j in i + 1..N {
                if no_flg {
                    tmp[j] = 'N';
                    continue;
                }
                if !I[j] {
                    tmp[j] = 'N';
                    no_flg = true;
                }
                if !O[j] {
                    no_flg = true;
                }
            }
            ans.push(tmp);
        }
        println!("Case #{}:", t);
        for v in ans {
            println!("{}", v.into_iter().collect::<String>())
        }
    }
}

#[allow(dead_code)]
fn solve() {}

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
