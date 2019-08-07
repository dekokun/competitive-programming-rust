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
    // switch num
    let switch_num: i32 = read();
    // bulb num
    let bulb_num: i32 = read();
    let mut map = HashMap::new();
    // bi: bulb index
    for bi in 1..bulb_num + 1 {
        let k: i32 = read();
        for _ in 1..k + 1 {
            // si: switch index
            let si: i32 = read();
            let entry = map.entry(si).or_insert(vec![]);
            entry.push(bi)
        }
    }
    let mut vec = vec![true];
    for _ in 1..bulb_num + 1 {
        vec.push(read::<i32>() == 0);
    }
    println!("{}", dfs(1, switch_num, &map, vec, 0));
}

fn dfs(
    depth: i32,
    last: i32,
    map: &HashMap<i32, Vec<i32>>,
    switch_state: Vec<bool>,
    ans: i32,
) -> i32 {
    if depth == last + 1 {
        if switch_state.iter().all(|&x| x == true) {
            return ans + 1;
        }
        return ans;
    }
    let mut switch_state_flip = switch_state.clone();
    match map.get(&depth) {
        Some(vec) => {
            for &i in vec {
                switch_state_flip[i as usize] ^= true;
            }
        }
        None => {}
    };
    let ans1 = dfs(depth + 1, last, &map, switch_state_flip, ans);
    let ans2 = dfs(depth + 1, last, &map, switch_state, ans);
    ans1 + ans2
}
