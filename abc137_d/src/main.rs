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
    let n: i32 = read();
    let m: i32 = read();
    let mut rewards = vec![];
    let mut set = std::collections::HashMap::new();
    for _ in 0..n {
        let reward = read::<i32>();
        let days = read::<i32>();
        set.entry(reward).or_insert(vec![]).push(days);
        rewards.push(reward);
    }
    let mut map = std::collections::HashMap::new();
    for reward in &rewards {
        let mut dayss = set.remove(reward).unwrap();
        dayss.sort();
        dayss.reverse();
        map.insert(reward.clone(), dayss);
    }
    rewards.sort();
    rewards.reverse();
    let mut total_reward = 0;
    for i in 1..m {
        for reward in &rewards {
            for days in map[reward].clone() {}
        }
    }
    println!("{}", total_reward);
}
