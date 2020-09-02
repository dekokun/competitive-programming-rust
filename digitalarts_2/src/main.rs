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
    let S: String = read();
    if S == "a" || S == "zzzzzzzzzzzzzzzzzzzz" {
        println!("NO");
        return;
    }
    let mut hash: usize = 0;
    for c in S.chars() {
        hash += (c as u8 - b'a' + 1) as usize;
    }
    let mut ans = "".to_owned();
    while hash > 0 {
        dbg!(hash);
        match (hash / 26) as u8 {
            0 => {
                let a = (hash % 26) as u8;
                ans.push((a + b'a' - 1) as char);
                hash -= a as usize;
            }
            _ => {
                ans.push('z');
                hash -= 26;
            }
        }
    }
    if S != ans {
        dbg!(1);
        println!("{}", ans);
        return;
    }
    if ans.chars().all(|c| c == 'z') {
        dbg!(2);
        ans.pop();
        // 全部zは最上段で防いでいるので必ず1文字入れる
        ans.push('y');
        ans.push('a');
        println!("{}", ans);
        return;
    }
    if ans.len() == 1 {
        dbg!(3);
        let c = ans.pop().unwrap();
        // 'a' は最上段で防いでいるので必ずb以上z以下
        ans.push((c as u8 - 1) as char);
        ans.push('a');
        println!("{}", ans);
        return;
    }
    let mut ans = "".to_owned();
    let mut minus = false;
    let mut plus = false;
    for c in S.chars() {
        if c != 'z' && !plus {
            ans.push((c as u8 + 1) as char);
            plus = true;
        } else if c != 'a' && !minus {
            ans.push((c as u8 - 1) as char);
            minus = true;
        } else {
            ans.push(c);
        }
    }
    println!("{}", ans)
}
