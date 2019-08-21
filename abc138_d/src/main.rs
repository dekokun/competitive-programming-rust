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
    let n: usize = read();
    let q: usize = read();
    let mut tree = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let a: usize = read::<usize>() -1;
        let b: usize = read::<usize>() -1;
        tree[a].push(b);
        tree[b].push(a);
    }
    let mut ans = vec![0; n];
    for _ in 0..q {
        // operation vertex
        let p: usize = read::<usize>() - 1;
        // add value
        let x: usize = read::<usize>();
        ans[p] += x;
    }
    get_values(&tree, 0, 0, &mut ans);
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
    tree: &Vec<Vec<usize>>,
    vertex: usize,
    before: usize,
    ans: &mut Vec<usize>,
) {
    for vert in &tree[vertex] {
        if *vert == before {
            continue;
        }
        ans[*vert] += ans[vertex];
        get_values(&tree, *vert, vertex, ans)
    }
}
