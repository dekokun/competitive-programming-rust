use std::io::*;
use std::str::FromStr;

#[allow(dead_code)]
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

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let opt = read_option();
    opt.expect("failed to parse token")
}

// binary heap for PartialOrd
struct MyBinaryHeap<T> {
    data: Vec<T>,
}
// TODO: add impl for Debug
// TODO: add impl for Clone
impl<T: PartialOrd + Copy> MyBinaryHeap<T> {
    pub fn new() -> MyBinaryHeap<T> {
        MyBinaryHeap { data: vec![] }
    }
    // TODO: panic if partial_cmp return None
    pub fn push(&mut self, item: T) {
        self.data.push(item);
        let mut now_index = self.data.len() - 1;
        while now_index > 0 {
            // startは1か0か。0にしてみる
            let parent_index = (now_index - 1) / 2;
            if self.data[now_index] <= self.data[parent_index] {
                break;
            }
            let now = self.data[now_index];
            let parent = self.data[parent_index];
            self.data[now_index] = parent;
            self.data[parent_index] = now;
            now_index = parent_index;
        }
    }
    // TODO: panic if partial_cmp return None
    pub fn pop(&mut self) -> Option<T> {
        let item = self.data.pop();
        if self.data.len() == 0 {
            return item;
        }
        let ret = self.data[0];
        self.data[0] = item.unwrap();
        let mut now_index = 0;
        loop {
            // startは1か0か。0にしてみる
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
            if self.data[now_index] >= self.data[change_index] {
                break;
            }
            let now = self.data[now_index];
            let child = self.data[change_index];
            self.data[now_index] = child;
            self.data[change_index] = now;
            now_index = change_index;
        }
        Some(ret)
    }
}
// slow because when call next, popped every time.
impl<T: PartialOrd + Copy> Iterator for MyBinaryHeap<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.pop()
    }
}

#[test]
fn it_works() {
    let mut a = MyBinaryHeap::new();
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
    let mut a = MyBinaryHeap::new();
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
fn main() {
    let mut queue = MyBinaryHeap::new();
    let n: i32 = read();
    let m: i64 = read();
    for _ in 0..n {
        let price: i64 = read();
        queue.push(price);
    }
    for _ in 0..m {
        let price = queue.pop().unwrap();
        // round
        queue.push(price / 2);
    }
    let mut sum = 0;
    for price in queue {
        sum += price;
    }
    println!("{}", sum);
}
