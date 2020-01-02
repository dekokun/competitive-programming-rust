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

fn gcd(min: usize, max: usize) -> usize {
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
        println!("{}", std::cmp::max(read::<usize>(), read::<usize>()));
        return;
    }
    let mut vec: Vec<usize> = vec![];
    for _ in 0..n {
        vec.push(read());
    }
    let mut gcd_l = vec![0; n];
    let mut now_l_gcd = vec[0];
    let mut gcd_r = vec![0; n];
    let mut now_r_gcd = vec[n - 1];
    for i in 0..n {
        gcd_l[i] = gcd(now_l_gcd, vec[i]);
        now_l_gcd = gcd_l[i];
        gcd_r[n - i - 1] = gcd(now_r_gcd, vec[n - i - 1]);
        now_r_gcd = gcd_r[n - i - 1];
    }
    let mut max = 1;
    for i in 0..n {
        let gcd = if i == 0 {
            gcd_r[i + 1]
        } else if i == n - 1 {
            gcd_l[i - 1]
        } else {
            gcd(gcd_l[i - 1], gcd_r[i + 1])
        };
        max = std::cmp::max(gcd, max);
    }
    println!("{}", max);
}
