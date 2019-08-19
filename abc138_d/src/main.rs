use std::collections::HashMap;
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
    let q: i64 = read();
    let mut tree = HashMap::new();
    tree.insert(1, vec![]);
    for _ in 0..(n - 1) {
        let a: i64 = read();
        let b: i64 = read();
        let min = std::cmp::min(a, b);
        let max = std::cmp::max(a, b);
        {
            let entry = tree.entry(min).or_insert(vec![]);
            entry.push(max);
        }
        tree.entry(max).or_insert(vec![]);
    }
    let mut add_values = vec![0; n as usize];
    for _ in 0..q {
        // operation vertex
        let p: i64 = read();
        // add value
        let x: i64 = read();
        add_values[(p - 1) as usize] += x;
    }
    let mut ans = vec![0; n as usize];
    get_values(&tree, &add_values, 1, 0, n, &mut ans);
    println!(
        "{}",
        ans
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
fn get_values(
    tree: &HashMap<i64, Vec<i64>>,
    add_values: &Vec<i64>,
    vertex: i64,
    plus_val: i64,
    max: i64,
    ans: &mut Vec<i64>,
) {
    let mut plus_val = plus_val;
    if vertex == max + 1 {
        return;
    }
    plus_val += add_values[(vertex - 1) as usize];
    ans[(vertex - 1) as usize] =  plus_val;
    match tree.get(&vertex) {
        Some(vec) => {
            for &vert in vec {
                get_values(&tree, &add_values, vert, plus_val, max, ans)
            }
        }
        None => {}
    }
}
