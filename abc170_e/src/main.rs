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
    let n: usize = read();
    let q: usize = read();
    use std::collections::BTreeSet;
    use std::collections::HashMap;
    // kindergartenid: {(rate, infants id)}
    let mut kindergartens: HashMap<usize, BTreeSet<(usize, usize)>> = HashMap::new();
    // infants id: kindrgarten id
    let mut infants_kindergarten: HashMap<usize, usize> = HashMap::new();
    // infants id: rate
    let mut infants: HashMap<usize, usize> = HashMap::new();
    for infants_id in 1..=n {
        let (rate, kindergarten): (usize, usize) = (read(), read());
        let entry = kindergartens
            .entry(kindergarten)
            .or_insert_with(BTreeSet::new);
        entry.insert((rate, infants_id));
        infants_kindergarten.insert(infants_id, kindergarten);
        infants.insert(infants_id, rate);
    }
    // {(rate, infants_id)}
    let mut highests: BTreeSet<(usize, usize)> = BTreeSet::new();
    for (_k, v) in kindergartens {
        let &max = v.iter().next().unwrap();
        highests.insert(max);
    }
    for _ in 0..q {
        let (infant_id, next_kindergarten): (usize, usize) = (read(), read());
        // 保育園の今のidを取得
        // kindergartensの書き換え(旧保育園)
        // kindergartensの書き換え(新保育園)
        // infant id とkindergartenの対応書き換え
        // infantのid取得
        // hiestsの
    }
}
