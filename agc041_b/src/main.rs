#![allow(non_snake_case)]

use std::collections::HashMap;
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

// 考察
// 下限を上の曖昧ゾーンにいかに突っ込むか
// 1つの問題は最大、ジャッジの数だけプラスされる。最低は0
// 下限に全ジャッジの数プラスされて曖昧ゾーンに突っ込むか
// 曖昧ゾーンは必ず今の曖昧ゾーン以上になる
// 上がどれだけ高くなってもよくて、Pの最後の1個が一番小さいのはいくつかわかると良い
// 上位P - 1の問題には全部投票されたと考えて良い？
// VがP以下だと簡単。今の上位のV - 1個にM票が集まりつつ、一番小さいやつにM票が集まってPに入れるかどうか
//   というか、今の上位のV - 1個にM票が集まった際のP番目の値 - M以上だったらいけるということや
// VがPより多いと、今の上位のP - 1個にM票が集まりつつ、一番小さいやつにM票が入りつつ、残りが間にいい感じに分散した時にPに入れるかどうか
//   残りV - (P - 1)個をM人が分配する。これを小さいやつから順に配布してどこまで入るかで、入る人はわかる
//   問題はそこで入ってない人でも、ギリギリ入った人にマックス投票されなかった場合に入り得るということ
//   そのシミュレーションは、ギリギリ入った人及びまだ入ってない全員の最大値が低くなるように分配された場合に入るかどうか

fn main() {
    let N: usize = read();
    let M: usize = read();
    let V: usize = read();
    let P: usize = read();
    let mut vec: Vec<usize> = vec![];
    for _ in 0..N {
        let v = read();
        vec.push(v);
    }
    vec.sort();
    let mut reversed_vec = vec.clone();
    reversed_vec.reverse();
    let mut scores: HashMap<usize, (usize, usize)> = HashMap::new();
    for (i, &v) in vec.iter().enumerate() {
        (*scores.entry(v).or_insert((i, i))).1 = i;
    }
    if V <= P {
        // P番目。これ以上になれば良い
        let min = reversed_vec[P - 1] - M;
        match vec.binary_search(&min) {
            Ok(a) => {
                let (min, _) = scores[&vec[a]];
                println!("{}", N - min);
                return;
            }
            Err(a) => {
                let (min, _) = scores[&vec[a]];
                println!("{}", N - min);
                return;
            }
        }
    }
}
