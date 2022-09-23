use std::collections::VecDeque;

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: u32
    }

    let mut vec_deque: VecDeque<String> = VecDeque::new();
    for _ in 0..10 {
        let m = N % 2;
        vec_deque.push_front(m.to_string());
        N /= 2;
    }
    let vec = Vec::from(vec_deque);
    println!("{:?}", vec.join(""));
}
