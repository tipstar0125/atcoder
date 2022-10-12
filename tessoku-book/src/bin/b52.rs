use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        _: usize,
        X: usize,
        mut A: Chars
    }

    let mut queue = VecDeque::new();
    queue.push_back(X - 1);
    A[X - 1] = '@';

    while !queue.is_empty() {
        let pos = *queue.front().unwrap();

        if pos > 0 && A[pos - 1] == '.' {
            A[pos - 1] = '@';
            queue.push_back(pos - 1);
        }
        if pos < A.len() - 1 && A[pos + 1] == '.' {
            A[pos + 1] = '@';
            queue.push_back(pos + 1);
        }

        queue.pop_front();
    }
    println!("{}", A.iter().join(""));
}
