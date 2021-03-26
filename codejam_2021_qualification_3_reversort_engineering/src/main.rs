#![allow(non_snake_case, unused_macros, dead_code)]

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
    let t: usize = read();
    for i in 1..=t {
        let n: usize = read();
        let c: usize = read();
        println!("Case #{}: {}", i, solve(n, c));
    }
}

fn solve(n: usize, c: usize) -> String {
    // 最低は、n - 1 (最初から昇順に並んでる場合)
    // 例： 1 2 3 4
    // ここから、1増やすにはどうする？ -> 1 2 4 3
    // 更に1増やすには？ ->
    // 最高は、n + (n - 1) + ... + 2 = ((n + 1)n / 2) - 1
    let max = ((n + 1) * n) / 2 - 1;
    let min = n - 1;
    if c < min || max < c {
        return "IMPOSSIBLE".into();
    }
    // 例1 2 4 3 1 -> 1 3 4 2 -> 1 2 4 3 -> 1 2 3 4
    //コスト 9(max)  6(9 - 4 + 1)  4(6 - 3 + 1) 3(4 - 2 + 1)
    // コスト5は？ 1 3 4 2 から、1減らす。 1 4 3 2
    // コスト7は？ 2 4 3 1 から、2減らす(ということは、3個の入れ替えをやめれば良い)。ということは、1回目の操作の後に 1 2 4 3になっていれば良い
    // -> 3 4 2 1 -> 1 2 4 3(cost 4) ->  1 2 4 3(cost 5) -> 1 2 3 4(cost 7)
    // 例2 2 4 6 5 3 1 -> 1 3 5 6 4 2 -> 1 2 4 6 5 3 -> 1 2 3 5 6 4 -> 1 2 3 4 6 5 -> 1 2 3 4 5 6
    // 足す方向で考えたほうがかんたん。
    // コスト5は？ 1 2 4 3 に1足す(=初回に2の回転を入れる) -> 2 1 4 3 cost 5(= 2 + 1 + 2)
    // コスト8は？ 1 3 4 2 に2足す(=初回に3の回転を入れる) -> 4 3 1 2 cost 8(=3 + 3 + 2)
    let mut now = max;
    let mut index = 0;
    let mut before_cost = 0;
    for i in 0..n {
        debug!(now);
        if now <= c {
            before_cost = now;
            index = i;
            break;
        }
        now -= n - i - 1;
    }
    debug!(c, before_cost, index);
    let mut max_vec = vec![0; n];
    for i in 1..=n {
        if i % 2 == 0 {
            max_vec[i / 2 - 1] = i;
        } else {
            max_vec[n - i / 2 - 1] = i;
        }
    }
    debug!(max_vec);
    for i in 0..index {
        let sl = &max_vec[i..];
        let mut sl = sl.to_vec();
        sl.reverse();
        for (i2, v) in sl.into_iter().enumerate() {
            max_vec[i + i2] = v;
        }
    }
    debug!(max_vec);
    let gap = c - before_cost;
    let sl = &max_vec[..=gap];
    let mut sl = sl.to_vec();
    sl.reverse();
    for (i2, v) in sl.into_iter().enumerate() {
        max_vec[i2] = v;
    }
    max_vec
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
