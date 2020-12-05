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

// a: 遠ざかったら戻す必要があるがその操作はもうできない
// a => b: どの1回でもどれかが遠ざかるような動きはできない
// b => c: すでに自分のいるべき場所にいる数字があったらそれはもう動かせない
// なんとなく、小さい方からあるべき場所に移動させていけば良さそう
// すでに移動した後の移動が起きたら-1
fn main() {
    let n: usize = read();
    let mut vec: Vec<usize> = vec![];
    for _ in 0..n {
        vec.push(read::<usize>() - 1);
    }
    use std::collections::HashMap;
    // 値からposへのmap
    let mut map: HashMap<usize, usize> = HashMap::new();
    for (pos, &v) in vec.iter().enumerate() {
        map.insert(v, pos);
    }
    let mut passed: Vec<bool> = vec![false; n - 1];
    let mut ans = vec![];
    for i in 0..n {
        let mut pos = *map.get(&i).unwrap();
        while pos != i {
            if pos < i {
                println!("-1");
                return
            }
            let new_pos = pos - 1;
            if passed[new_pos] {
                println!("{}", -1);
                return
            }
            passed[new_pos] = true;
            let change_val = vec[new_pos];
            vec[new_pos] = i;
            vec[pos] = change_val;
            *map.get_mut(&i).unwrap() = new_pos;
            *map.get_mut(&change_val).unwrap() = pos;
            pos = new_pos;
            ans.push(new_pos);
        }
    }
    if ans.len() != n - 1 {
        println!("-1");
        return
    }
    for i in ans {
        println!("{}", i + 1);
    }
}
