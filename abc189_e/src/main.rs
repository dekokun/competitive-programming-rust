#![allow(non_snake_case)]

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
    let n = read();
    let xy = (0..n).map(|_| (read(), read())).collect();
    let m = read();
    let op = (0..m)
        .map(|_| {
            let op1 = read::<usize>();
            if op1 == 1 || op1 == 2 {
                (op1, 0)
            } else {
                (op1, read())
            }
        })
        .collect();
    let q = read();
    debug!(n, &xy, m, &op, q);

    println!(
        "{}",
        solve(n, xy, m, op, q, (0..q).map(|_| (read(), read())).collect()).join("\n")
    );
}

fn solve(
    _n: usize,
    xy: Vec<(i64, i64)>,
    _m: usize,
    op: Vec<(usize, i64)>,
    _q: usize,
    query: Vec<(usize, usize)>,
) -> Vec<String> {
    // xの係数, yの係数, その他定数
    let mut results = vec![((1, 0, 0), (0, 1, 0))];
    for (i, (op1, op2)) in op.into_iter().enumerate() {
        let before = results[i];
        let before_x = before.0;
        let x_x = before_x.0;
        let x_y = before_x.1;
        let x_c = before_x.2;
        let before_y = before.1;
        let y_x = before_y.0;
        let y_y = before_y.1;
        let y_c = before_y.2;
        results.push(match op1 {
            1 => ((y_x, y_y, y_c), (-x_x, -x_y, -x_c)),
            2 => ((-y_x, -y_y, -y_c), (x_x, x_y, x_c)),
            3 => ((-x_x, -x_y, -x_c + 2 * op2), (y_x, y_y, y_c)),
            4 => ((x_x, x_y, x_c), (-y_x, -y_y, -y_c + 2 * op2)),
            _ => unimplemented!("not implemented"),
        });
    }
    let mut ans = vec![];
    for (a, b) in query {
        let target = xy[b - 1];
        let x = target.0;
        let y = target.1;
        let op = results[a];
        let op_x = op.0;
        let op_y = op.1;
        ans.push(format!(
            "{} {}",
            x * op_x.0 + y * op_x.1 + op_x.2,
            x * op_y.0 + y * op_y.1 + op_y.2
        ));
    }

    ans
}
