#![allow(non_snake_case)]
// #![feature(non_ascii_idents)]

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
    let T: usize = read();
    for t in 1..T + 1 {
        let N: usize = read();
        let D: usize = read();
        let mut vec: Vec<i64> = vec![];
        for _ in 0..N {
            vec.push(read());
        }
        let ans = solve(D, vec);
        println!("Case #{}: {}", t, ans);
    }
}

// - あるキーよりも上のものがいくつあるかが重要
//   - ちょうど2倍のもの、2倍以上のもの、ちょうど3倍のもの、3倍以上のもの…がいくつあるかも重要
// - あるキーが何個あるのかを大きい順から見ていき、そのキーよりも上の数で何個新たにそのキーを作り出せるかを計算する
fn solve(diner: usize, vec: Vec<i64>) -> usize {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut half_map = HashMap::new();
    for v in vec {
        let entry = map.entry(v).or_insert(0);
        *entry += 1;
        {
            let entry = half_map.entry(v).or_insert(0);
            *entry += 1;
        }
        if v % 2 == 0 {
            let entry = half_map.entry(v / 2).or_insert(0);
            *entry += 2;
        }
    }
    let mut max = 0;
    let mut min_max_key = 0;
    let mut max_key = 0;
    for (k, v) in map {
        if max == v {
            min_max_key = std::cmp::min(min_max_key, k);
        } else if max < v {
            max = v;
            min_max_key = k;
        }
        max_key = std::cmp::max(max_key, k);
    }
    if max >= diner {
        0
    } else if diner - 1 == max {
        if diner == 2 {
            1
        // 2個あるのが最大なので増やせない
        } else if max_key == min_max_key {
            let mut max = 0;
            for (_k, v) in half_map {
                max = std::cmp::max(max, v);
            }
            if max >= diner {
                1
            } else {
                2
            }
        } else {
            1
        }
    // test case 1: 3人でmaxが1の場合
    } else {
        let mut max = 0;
        for (_k, v) in half_map {
            max = std::cmp::max(max, v);
        }
        if max >= diner {
            1
        } else {
            2
        }
    }
}

// #[test]
// fn it_works() {
//     assert_eq!(solve(3, vec![1]), 2);
// }
//
// #[test]
// fn it_works2() {
//     assert_eq!(solve(2, vec![10, 5, 359999999999, 123456789, 10]), 0);
// }
//
// #[test]
// fn 二等分すると同じになって3個になる() {
//     assert_eq!(solve(3, vec![8, 4]), 1);
// }
//
// #[test]
// fn it_works4() {
//     assert_eq!(solve(2, vec![1, 2, 3]), 1);
// }
//
// #[test]
// fn 二個あるやつが最大() {
//     assert_eq!(solve(3, vec![1, 3, 3]), 2);
// }
//
// #[test]
// fn 二個あるやつが最大かつ2当分すると同じになる() {
//     assert_eq!(solve(3, vec![1, 3, 3, 2]), 1);
// }
//
// #[test]
// fn 二個あるやつが最大ではない() {
//     assert_eq!(solve(3, vec![1, 2, 2, 3, 3]), 1);
// }
//
// #[test]
// fn 二個あるやつが最大ではない2() {
//     assert_eq!(solve(3, vec![1, 3, 3, 2, 2]), 1);
// }
//
