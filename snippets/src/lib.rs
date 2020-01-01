#![feature(plugin)]
#![plugin(cargo_snippet)]

#[snippet = "BinaryHeap"]
// binary heap for PartialOrd
struct BinaryHeap<T> {
    data: Vec<T>,
}
#[snippet = "BinaryHeap"]
impl<T: Clone> Clone for BinaryHeap<T> {
    fn clone(&self) -> Self {
        BinaryHeap {
            data: self.data.clone(),
        }
    }
}
#[snippet = "BinaryHeap"]
impl<T: fmt::Debug> core::fmt::Debug for BinaryHeap<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.data.iter()).finish()
    }
}
#[snippet = "BinaryHeap"]
impl<T: PartialOrd> BinaryHeap<T> {
    pub fn new() -> BinaryHeap<T> {
        BinaryHeap { data: vec![] }
    }
    pub fn push(&mut self, item: T) {
        self.data.push(item);
        let mut now_index = self.data.len() - 1;
        while now_index > 0 {
            // 0 start
            let parent_index = (now_index - 1) / 2;
            let cmp_result = self.data[now_index].partial_cmp(&self.data[parent_index]);
            if cmp_result.is_none() {
                panic!("compare non comparable values");
            }
            if cmp_result.unwrap() == Less || cmp_result.unwrap() == Equal {
                break;
            }
            self.data.swap(now_index, parent_index);
            now_index = parent_index;
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let ret = self.data.swap_remove(0);
        let mut now_index = 0;
        if self.data.is_empty() {
            return Some(ret);
        }
        loop {
            let child_index1 = (now_index + 1) * 2 - 1;
            let child_index2 = (now_index + 1) * 2;
            if child_index1 > self.data.len() - 1 {
                break;
            }
            let change_index = if child_index2 > self.data.len() - 1
                || self.data[child_index1] > self.data[child_index2]
            {
                child_index1
            } else {
                child_index2
            };
            let cmp_result = self.data[now_index].partial_cmp(&self.data[change_index]);
            if cmp_result.is_none() {
                panic!("compare non comparable");
            }
            if cmp_result.unwrap() == Greater || cmp_result.unwrap() == Equal {
                break;
            }
            self.data.swap(now_index, change_index);
            now_index = change_index;
        }
        Some(ret)
    }
}
// slow because when call next, popped every time.
#[snippet = "BinaryHeap"]
impl<T: PartialOrd> Iterator for BinaryHeap<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.pop()
    }
}

#[test]
fn it_works() {
    let mut a = BinaryHeap::new();
    assert_eq!(a.pop(), None);
    a.push(1);
    a.push(5);
    a.push(0);
    a.push(100);
    assert_eq!(a.pop(), Some(100));
    assert_eq!(a.pop(), Some(5));
    assert_eq!(a.pop(), Some(1));
    assert_eq!(a.pop(), Some(0));
    assert_eq!(a.pop(), None);
    assert_eq!(a.pop(), None);
}

#[test]
fn iter_works() {
    let mut a = BinaryHeap::new();
    assert_eq!(a.pop(), None);
    a.push(1);
    a.push(5);
    a.push(0);
    a.push(100);
    let expected = vec![100, 5, 1, 0];
    for (i, v) in a.enumerate() {
        assert_eq!(expected[i], v);
    }
}
#[test]
#[should_panic]
fn nan_panic() {
    let mut a = BinaryHeap::new();
    a.push(1.5);
    a.push(std::f64::NAN);
}

#[snippet = "read_option"]
use std::io::{stdin, Read};
#[snippet = "read_option"]
use std::str::FromStr;

#[snippet = "read_option"]
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

#[snippet(include = "read_option")]
#[snippet = "read"]
fn read<T: FromStr>() -> T {
    let opt = read_option();
    opt.expect("failed to parse token")
}
