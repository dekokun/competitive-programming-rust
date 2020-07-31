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
    let s: Vec<char> = read::<String>().chars().collect();
    let mut deck: Vec<(char, String)> = vec![];
    let mut i = 0;
    while i < s.len() {
        let suit = s[i];
        i += 1;
        let first = s[i];
        i += 1;
        if first == '1' {
            i += 1;
            deck.push((suit, "10".to_owned()));
        } else {
            deck.push((suit, first.to_string()));
        }
    }
    use std::collections::HashMap;
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut gatherSuit = ' ';
    for (suit, num) in &deck {
        match num.as_ref() {
            "10" | "J" | "Q" | "K" | "A" => {
                let entry = map.entry(*suit).or_insert(0);
                *entry += 1;
                if *entry == 5 {
                    gatherSuit = *suit;
                    break;
                }
            }
            _ => {
                continue;
            }
        }
    }
    let mut ans = vec![];
    let mut count = 0;
    for (suit, num) in &deck {
        if suit != &gatherSuit {
            ans.push((suit, num));
            continue;
        }
        match num.as_ref() {
            "10" | "J" | "Q" | "K" | "A" => {
                count += 1;
                if count == 5 {
                    break;
                }
            }
            _ => {
                ans.push((suit, num));
            }
        }
    }
    if ans.is_empty() {
        println!("0")
    } else {
        println!(
            "{}",
            ans.into_iter()
                .map(|(suit, num)| format!("{}{}", suit, num))
                .collect::<Vec<_>>()
                .join("")
        );
    }
}
