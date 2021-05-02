#![allow(non_snake_case, unused_macros, dead_code)]

// https://maguro.dev/debug-macro/ $+$i
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

use std::{collections::HashSet, str::FromStr};
use std::{
    collections::VecDeque,
    io::{stdin, Read},
};
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
        println!("Case #{}: {}", t, solve(read(), read()));
    }
}

fn solve(s: String, e: String) -> String {
    // 101 -> 01, 1010
    // if s == e {
    //     return "0".into();
    // }
    // e: 0の場合
    // if e == "0".to_string() {
    //     let mut s = s;
    //     let mut count = 0;
    //     while s != "0" {
    //         s = not(&s);
    //         count += 1;
    //     }
    //     return count.to_string();
    //}
    // e: 111.. の場合
    // if e.chars().all(|c| c == '1') {
    //     let len = e.len();
    //     let s_right_zero_count = s.chars().rev().take_while(|&c| c == '0').count();
    //     let ans1 = if s_right_zero_count <= len {
    //         len - s_right_zero_count + 1
    //     } else {
    //         1 + len + 1
    //     };
    //     let s = not(&s);
    //     let s_right_zero_count = s.chars().rev().take_while(|&c| c == '0').count();
    //     let ans2 = 1 + if s_right_zero_count <= len {
    //         len - s_right_zero_count + 1
    //     } else {
    //         1 + len + 1
    //     };
    //     return ans1.min(ans2).to_string();
    // }
    // // IMPOSSIBLEの場合
    // let mut s2 = s.clone();
    // let e2: Vec<char> = e.chars().rev().skip_while(|&c| c == '0').collect();
    // let e2: String = e2.into_iter().rev().collect();

    // let mut count = 0;
    // while s2 != "0" {
    //     if s2 == e2 {
    //         return (e2.len() - s2.len() + count).to_string();
    //     }
    //     s2 = not(&s2);
    //     count += 1;
    // }
    // // eの逆転
    // let e2 = not(&e);
    // let mut s2 = s.clone();
    // let e2: Vec<char> = e2.chars().rev().skip_while(|&c| c == '0').collect();
    // let e2: String = e2.into_iter().rev().collect();
    // let mut count = 0;
    // while s2 != "0" {
    //     if s2 == e2 {
    //         return (e2.len() - s2.len() + count - 1).to_string();
    //     }
    //     s2 = not(&s2);
    //     count += 1;
    // }
    // return "IMPOSSIBLE".into();
    match bfs(s.clone(), &e, 0) {
        Some(a) => a.to_string(),
        None => "IMPOSSIBLE".into(),
    }
}

fn bfs(s: String, e: &String, count: u64) -> Option<u64> {
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    visited.insert(s.clone());
    q.push_front((s, e, count, 0));
    while let Some((s, e, count, plus_count)) = q.pop_back() {
        if &s == e {
            return Some(count);
        }
        let next = not(&s);
        if !visited.contains(&next) {
            visited.insert(next.clone());
            q.push_front((next, e, count + 1, plus_count));
        }
        if s == "0" {
            continue;
        }
        let next = s + "0";
        if (!visited.contains(&next)) && plus_count <= e.len() + 1 {
            visited.insert(next.clone());
            q.push_front((next, e, count + 1, plus_count + 1));
        }
    }
    None
}

fn not(s: &String) -> String {
    let ans = s
        .chars()
        .map(|c| match c {
            '1' => '0',
            '0' => '1',
            _ => panic!(),
        })
        .skip_while(|&c| c == '0')
        .collect::<String>();
    if ans == "" {
        "0".into()
    } else {
        ans
    }
}
