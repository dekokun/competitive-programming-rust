#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::HashMap;

use proconio::{input, marker::Chars};

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        t: usize,
    }
    for i in 1..=t {
        input! {
            s: Chars
        }
        println!("Case #{}: {}", i, solve(s));
    }
}

fn solve(s: Vec<char>) -> usize {
    let mut vowels = HashMap::new();
    let mut consonants = HashMap::new();
    for c in s {
        let change = if vec!['A', 'I', 'U', 'E', 'O'].contains(&c) {
            &mut vowels
        } else {
            &mut consonants
        };
        let entry = change.entry(c).or_insert(0);
        *entry += 1;
    }
    let vowel_count: usize = vowels.iter().map(|(_i, v)| v).sum();
    let consonants_count: usize = consonants.iter().map(|(_i, v)| v).sum();
    let consonants_max = consonants.iter().map(|(_i, v)| v).max();
    let vowel_max = vowels.iter().map(|(_i, v)| v).max();
    if vowel_count == 0 {
        return ((consonants_count - consonants_max.unwrap()) * 2).min(consonants_count);
    }
    if consonants_count == 0 {
        return ((vowel_count - vowel_max.unwrap()) * 2).min(vowel_count);
    }
    let consonants_max = consonants_max.unwrap();
    let vowel_max = vowel_max.unwrap();
    std::cmp::min(
        (consonants_count - consonants_max) * 2 + vowel_count,
        (vowel_count - vowel_max) * 2 + consonants_count,
    )
}
