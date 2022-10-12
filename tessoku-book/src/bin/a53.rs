use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        Q: usize,
    }

    let mut Query = vec![];
    for _ in 0..Q {
        input! {
            n: usize,
        }
        if n == 1 {
            input! {
               price: usize
            }
            Query.push((n, price));
        } else {
            Query.push((n, 0));
        }
    }

    let mut heap = BinaryHeap::new();
    for q in &Query {
        match q {
            (1, v) => heap.push(Reverse(v)),
            (2, _) => {
                if let Some(Reverse(v)) = heap.peek() {
                    println!("{:?}", v);
                }
            }
            (3, _) => {
                heap.pop();
            }
            (_, _) => unreachable!(),
        }
    }
}
