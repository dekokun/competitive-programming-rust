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
    let n: i64 = read();
    let mut a_s: Vec<i64> = vec![];
    let mut b_s: Vec<i64> = vec![];
    for _ in 0..(n + 1) {
        a_s.push(read());
    }
    for _ in 0..n {
        b_s.push(read());
    }
    let mut next_remain = a_s[0];
    let mut sum = 0;
    for (i, b) in b_s.into_iter().enumerate() {
        let now_remain = next_remain.clone();
        // ai 残存
        if b <= now_remain {
            sum += b;
            next_remain = a_s[i + 1];
            continue;
        }
        // ai 全滅
        sum += now_remain;
        let hero_remain = b - now_remain;

        // next全滅
        if a_s[i + 1] <= hero_remain {
            sum += a_s[i + 1];
            next_remain = 0;
            continue;
        }

        sum += hero_remain;
        next_remain = a_s[i + 1] - hero_remain;
    }
    println!("{}", sum);
    return;
}
