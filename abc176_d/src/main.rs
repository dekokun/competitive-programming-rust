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
    let H: usize = read();
    let W: usize = read();
    let CH: usize = read();
    let CW: usize = read();
    let DH: usize = read();
    let DW: usize = read();
    let mut map: Vec<Vec<char>> = vec![];
    for _ in 0..H {
        map.push(read::<String>().chars().collect());
    }
    let mut checked: Vec<Vec<Option<usize>>> = vec![vec![None; W]; H];
    let mut teams = vec![];
    use std::collections::HashSet;
    use std::collections::VecDeque;
    for i in 0..H {
        for j in 0..W {
            if checked[i][j].is_some() {
                continue;
            }
            if map[i][j] == '#' {
                continue;
            }
            let mut team = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_front((i, j));
            team.insert((i, j));
            let team_num = teams.len();
            checked[i][j] = Some(team_num);
            while let Some((h, w)) = queue.pop_back() {
                for (addH, addW) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let h = h as isize;
                    let w = w as isize;
                    if h + addH >= H as isize
                        || w + addW >= W as isize
                        || h + addH < 0
                        || w + addW < 0
                    {
                        continue;
                    }
                    let newH = (h + addH) as usize;
                    let newW = (w + addW) as usize;
                    if checked[newH][newW].is_some() {
                        continue;
                    }
                    if map[newH][newW] == '.' {
                        checked[newH][newW] = Some(team_num);
                        queue.push_front((newH, newW));
                        team.insert((newH, newW));
                    }
                }
            }
            teams.push(team);
        }
    }
    use std::collections::HashMap;
    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
    for h in 0..H {
        for w in 0..W {
            let now_team = checked[h][w];
            if now_team.is_none() {
                continue;
            }
            let now_team = now_team.unwrap();
            for addH in -2..=2 {
                for addW in -2..=2 {
                    let h = h as isize;
                    let w = w as isize;
                    if h + addH >= H as isize
                        || w + addW >= W as isize
                        || h + addH < 0
                        || w + addW < 0
                    {
                        continue;
                    }
                    let newH = (h + addH) as usize;
                    let newW = (w + addW) as usize;
                    let new_team = checked[newH][newW];
                    if new_team.is_none() {
                        continue;
                    }
                    let new_team = new_team.unwrap();
                    if new_team != now_team {
                        let entry = graph.entry(now_team).or_insert_with(HashSet::new);
                        entry.insert(new_team);
                    }
                }
            }
        }
    }
    let mut start_team = 0;
    let mut goal_team = 0;
    for (i, team) in teams.into_iter().enumerate() {
        if team.contains(&(CH - 1, CW - 1)) {
            start_team = i;
        }
        if team.contains(&(DH - 1, DW - 1)) {
            goal_team = i;
        }
    }
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_front((start_team, 0));
    visited.insert(start_team);
    while let Some((now_team, cost)) = queue.pop_back() {
        if now_team == goal_team {
            println!("{}", cost);
            return;
        }
        match graph.get(&now_team) {
            None => continue,
            Some(set) => {
                for &next_team in set {
                    if visited.contains(&next_team) {
                        continue;
                    }
                    queue.push_front((next_team, cost + 1));
                    visited.insert(next_team);
                }
            }
        }
    }
    println!("-1");
}
