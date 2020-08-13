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
    let map: Vec<Vec<char>> = (0..H).map(|_| read::<String>().chars().collect()).collect();
    let mut check_map = vec![vec![false; W]; H];
    use std::collections::VecDeque;
    let mut parts: Vec<Vec<char>> = vec![];
    for i in 0..H {
        for j in 0..W {
            if check_map[i][j] {
                continue;
            }
            let mut queue = VecDeque::new();
            queue.push_front((i, j));
            check_map[i][j] = true;
            let mut part = vec![];
            part.push(map[i][j]);
            while let Some((y, x)) = queue.pop_back() {
                for (add_y, add_x) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let y = y as i32;
                    let x = x as i32;
                    if y + add_y < 0
                        || y + add_y >= H as i32
                        || x + add_x < 0
                        || x + add_x >= W as i32
                        || check_map[(y + add_y) as usize][(x + add_x) as usize]
                        || map[y as usize][x as usize]
                            == map[(y + add_y) as usize][(x + add_x) as usize]
                    {
                        continue;
                    }
                    let y = (y + add_y) as usize;
                    let x = (x + add_x) as usize;
                    queue.push_front((y, x));
                    check_map[y][x] = true;
                    part.push(map[y][x]);
                }
            }
            parts.push(part);
        }
    }
    let sum = parts.into_iter().fold(0, |acc, part| {
        let black_count = part.iter().filter(|&&x| x == '#').count();
        let white_count = part.iter().filter(|&&x| x == '.').count();
        acc + black_count * white_count
    });
    println!("{}", sum)
}
