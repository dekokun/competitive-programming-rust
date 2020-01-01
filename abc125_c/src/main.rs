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

fn gcd(min: i32, max: i32) -> i32 {
    if min == 0 {}
    let r = max % min;
    if r == 0 {
        return min;
    }
    gcd(r, min)
}

fn main() {
    let n: usize = read();
    if n == 2 {
        println!("{}", std::cmp::max(read::<i32>(), read::<i32>()));
        return;
    }
    let mut vec = vec![];
    for _ in 0..n {
        vec.push(read());
    }
    let mut max: i32 = 0;
    for i in 0..n {
        let mut now_gcd: i32 = vec[i];
        for j in 0..n {
            // jがi-1の時以外(0の時はn-1)
            // 上で最初の最大公約数であるvec[i]をとっており、そのi以外の何か1つを避けるためにこうしている。最大公約数になりえるものを取ろうとしてこうなった模様(最小公倍数だったら1でいいんでけど)
            if (i != 0 && i - 1 != j) || (i == 0 && j != n - 1) {
                let r = vec[j];
                now_gcd = gcd(r, now_gcd);
            }
        }
        max = std::cmp::max(max, now_gcd);
    }
    println!("{}", max);
}
