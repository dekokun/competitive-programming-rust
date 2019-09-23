use rand::prelude::*;
// h w a b
fn make_random_vec() -> (usize, usize, usize, usize, Vec<Vec<usize>>) {
    let mut rng = rand::thread_rng();
    let h = 4;
    let w = 4;
    dbg!(h, w);
    let mut vec = vec![vec![0; w]; h];
    'outer: loop {
        // randomize
        for _ in 0..1000 {
            let a = rng.gen_range(0, w);
            let b = rng.gen_range(0, h);
            if vec[b][a] == 0 {
                vec[b][a] = 1;
            } else {
                vec[b][a] = 0;
            }
        }
        // a check
        let mut before_a = None;
        for row in vec.iter() {
            let mut a_count = 0;
            for v in row.iter() {
                if *v == 1 {
                    a_count += 1;
                }
            }
            let val = std::cmp::min(a_count, w - a_count);
            if before_a.is_some() && val != before_a.unwrap() {
                continue 'outer;
            }
            before_a = Some(val);
        }
        // b check
        let mut b_counts = vec![0; w];
        for row in vec.iter() {
            for (i, v) in row.iter().enumerate() {
                if *v == 1 {
                    b_counts[i] += 1;
                }
            }
        }
        let mut before_b = None;
        for v in b_counts {
            let val = std::cmp::min(v, h - v);
            if before_b.is_some() && val != before_b.unwrap() {
                continue 'outer;
            }
            before_b = Some(val);
        }
        return (h, w, before_a.unwrap(), before_b.unwrap(), vec);
    }
}
fn main() {
    let (h, w, a, b, vec) = make_random_vec();
    println!(
        "t \"{} {} {} {}\" \"{}\"",
        h,
        w,
        a,
        b,
        vec.iter()
            .map(|row| row
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(""))
            .collect::<Vec<_>>()
            .join("\n")
    )
}
