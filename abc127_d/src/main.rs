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

fn main() {
    let N: i32 = read();
    let M: i32 = read();
    let mut cards: Vec<i32> = vec![];
    let mut replaces: Vec<(i32, i32)> = vec![];

    for _ in 0..N {
        cards.push(read());
    }
    cards.sort();
    cards.reverse();

    for _ in 0..M {
        let b: i32 = read();
        let c: i32 = read();
        replaces.push((b, c));
    }
    replaces.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    replaces.reverse();

    let mut sum: i64 = 0;
    for (count, num) in replaces {
        for _ in 0..count {
            let card = cards.pop().unwrap();
            if card < num {
                sum += num as i64;
            } else {
                sum += card as i64;
            }
            if cards.len() == 0 {
                println!("{}", sum);
                return;
            }
        }
    }
    for card in cards {
        sum += card as i64;
    }
    println!("{}", sum);
}
