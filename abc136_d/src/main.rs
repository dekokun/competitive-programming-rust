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

fn main() {
    let s: String = read();
    let mut gather_indexes = vec![];
    let mut before = 'R';
    for (i, c) in s.chars().enumerate().skip(1) {
        if before == 'R' && c == 'L' {
            gather_indexes.push(i - 1);
        }
        before = c;
    }
    let mut gather_index = 0;
    let mut ans = vec![0; s.len()];
    for (i, c) in s.chars().enumerate() {
        // dbg!(s.clone(), c, i, gather_index, gather_indexes.clone());
        if c == 'L' {
            if (std::cmp::max(gather_indexes[gather_index], i)
                - std::cmp::min(gather_indexes[gather_index], i))
                % 2
                == 0
            {
                ans[gather_indexes[gather_index]] += 1;
            } else {
                ans[gather_indexes[gather_index] + 1] += 1;
            }
        } else {
            if gather_indexes[gather_index] == i {
                ans[gather_indexes[gather_index]] += 1;
            // dbg!("1");
            } else if gather_indexes[gather_index] > i {
                if (std::cmp::max(gather_indexes[gather_index], i)
                    - std::cmp::min(gather_indexes[gather_index], i))
                    % 2
                    == 0
                {
                    ans[gather_indexes[gather_index]] += 1;
                // dbg!("2");
                } else {
                    ans[gather_indexes[gather_index] + 1] += 1;
                    // dbg!("3");
                }
            } else if (std::cmp::max(gather_indexes[gather_index + 1], i)
                - std::cmp::min(gather_indexes[gather_index + 1], i))
                % 2
                == 0
            {
                ans[gather_indexes[gather_index + 1]] += 1;
            // dbg!("2");
            } else {
                ans[gather_indexes[gather_index + 1] + 1] += 1;
                // dbg!("3");
            }
        }
        // dbg!(ans.clone());
        if gather_index == gather_indexes.len() - 1 {
            continue;
        }
        if gather_indexes[gather_index + 1] == i + 1 {
            gather_index += 1;
        }
    }
    println!(
        "{}",
        ans.iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
