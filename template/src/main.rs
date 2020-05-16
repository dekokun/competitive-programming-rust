#![allow(non_snake_case)]

fn main() {}

#[allow(dead_code)]
fn solve() {}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, true);
    }
}
