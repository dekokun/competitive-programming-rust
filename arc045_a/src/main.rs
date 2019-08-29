use std::io::*;
use std::str::FromStr;

fn readOption<T: FromStr>() -> Option<T> {
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
    let opt = readOption();
    opt.expect("failed to parse token")
}

fn read2() -> Option<String> {
    let opt = readOption();
    match opt {
        Some(s) => {
            if s != "" {
                Some(s)
            } else {
                None
            }
        }
        None => None,
    }
}

fn main() {
    let mut vec = vec![];
    let mut map = std::collections::HashMap::new();
    map.insert("Left".to_string(), "<");
    map.insert("Right".to_string(), ">");
    map.insert("AtCoder".to_string(), "A");
    while let Some(s) = read2() {
        vec.push(map[&s]);
    }
    println!("{}", vec.join(" "));
}
