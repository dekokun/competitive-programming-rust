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
    let H: i32 = read();
    let W: i32 = read();
    let mut vec: Vec<Vec<char>> = vec![];
    for _ in 0..H {
        vec.push(read::<String>().chars().collect())
    }
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    let firstCost = if vec[0][0] == '#' { 1 } else { 0 };

    let mut costs = vec![vec![100 * 100 + 1; W as usize]; H as usize];
    queue.push_front((0, 0));
    costs[0][0] = firstCost;

    while let Some((row, column)) = queue.pop_back() {
        let cost = costs[row as usize][column as usize];
        if row == H - 1 && column == W - 1 {
            println!("{}", cost);
            return;
        }
        let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        for (dx, dy) in directions {
            if row + dy < 0 || H <= row + dy || column + dx < 0 || W <= column + dx {
                continue;
            }
            let newRow: usize = (row + dy) as usize;
            let newColumn: usize = (column + dx) as usize;
            let newCost =
                if vec[row as usize][column as usize] == '.' && vec[newRow][newColumn] == '#' {
                    cost + 1
                } else {
                    cost
                };
            if costs[newRow][newColumn] <= newCost {
                continue;
            }
            costs[newRow][newColumn] = newCost;
            queue.push_front((newRow as i32, newColumn as i32));
        }
    }
}
