#![allow(non_snake_case, unused_macros)]

// https://maguro.dev/debug-macro/ $B$+$i(B
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

use std::{cmp::Reverse, collections::BinaryHeap, str::FromStr};
use std::{
    collections::HashMap,
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
    let n: usize = read();
    let m: usize = read();
    let x: usize = read();
    let y: usize = read();
    let ans = solve(
        n,
        m,
        x,
        y,
        (0..m).map(|_| (read(), read(), read(), read())).collect(),
    );

    println!(
        "{}",
        if let Some(a) = ans {
            a.to_string()
        } else {
            "-1".into()
        }
    );
}

fn solve(
    n: usize,
    _m: usize,
    x: usize,
    y: usize,
    abtk: Vec<(usize, usize, u64, u64)>,
) -> Option<u64> {
    let mut map: HashMap<usize, Vec<(usize, u64, u64)>> = HashMap::new();
    for (a, b, t, k) in abtk {
        let entry = map.entry(a).or_insert_with(|| vec![]);
        entry.push((b, t, k));
        let entry = map.entry(b).or_insert_with(|| vec![]);
        entry.push((a, t, k));
    }
    let mut heap = BinaryHeap::new();
    let mut dist = vec![std::u64::MAX; n + 1];
    // (time, pos)
    heap.push((Reverse(0), x));
    dist[x] = 0;
    while let Some((Reverse(time), pos)) = heap.pop() {
        if pos == y {
            debug!(dist);
            return Some(time);
        }
        if time > dist[pos] {
            continue;
        }
        if !map.contains_key(&pos) {
            continue;
        }
        for &(next, cost, k) in &map[&pos] {
            let to_departure = if time % k == 0 { 0 } else { k - (time % k) };
            let next_time = time + to_departure + cost;
            if next_time < dist[next] {
                heap.push((Reverse(next_time), next));
                dist[next] = next_time;
            }
        }
    }
    None
}

/*                    -> 3(ボツ)
6(0) -> 2(6) -> 8(16) -> 4(19) -> 9(20)
             -> 3(16)
*/
