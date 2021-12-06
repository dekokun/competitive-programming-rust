#![allow(non_snake_case, unused_macros, dead_code)]

use proconio::input;

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        s: String,
    }
    println!("{}", solve(s));
}

fn solve(s: String) -> String {
    if s.len() == 0 {
        return "Yes".into();
    }
    if s.len() == 2 {
        match &s[0..] {
            "ox" | "xx" | "xo" => return "Yes".into(),
            _ => return "No".into(),
        };
    }

    let s = match &s[0..2] {
        "ox" => s,
        "xx" => "o".to_owned() + &s,
        "xo" => "ox".to_owned() + &s,
        _ => return "No".into(),
    };
    let s = match &s[s.len() - 2..] {
        "ox" => s + "x",
        "xx" => s,
        "xo" => s + "xx",
        _ => return "No".into(),
    };
    debug!(s);
    for i in 0..s.len() {
        if i % 3 != 0 {
            continue;
        }
        if &s[i..i + 3] != "oxx" {
            return "No".into();
        }
    }
    return "Yes".into();
}
