use std::io::*;
use std::str::FromStr;

//not passed
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
    let n: i8 = read();
    let mut v: Vec<i64> = vec![];
    for _ in 0..n {
        v.push(read());
    }
    while v.len() != 2 {
        let v2 = v.clone();
        // if v.len() == 3 {
        //     v[0] += v[1];
        //     v[2] += v[1];
        //     v.remove(1);
        //     continue;
        // }
        for (i, x) in v2.iter().enumerate() {
            let after = if i == (v2.len() - 1) {
                0
            } else if i == (v2.len() - 2) {
                10000000
            } else {
                v2[i + 1]
            };
            let before = if i == 0 {
                0
            // } else if i == 1 {
            //     10000000
            } else {
                v2[i - 1]
            };
            eprintln!("v: {:?}", v);
            eprintln!("{} {} {} ", before, x, after);
            if before >= *x && *x <= after {
                eprintln!("before: {:?}", v);
                eprintln!("remove: {}", x);
                v[i - 1] += *x;
                v[i + 1] += *x;
                v.remove(i);
                eprintln!("after: {:?}", v);
                break;
            }
        }
    }
    println!("{}", v[0] + v[1]);
}
