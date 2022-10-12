use std::collections::VecDeque;

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
               name: String
            }
            Query.push((n, name));
        } else {
            Query.push((n, "".to_string()));
        }
    }

    let mut queue = VecDeque::new();
    for q in &Query {
        match q {
            (1, v) => queue.push_back(v),
            (2, _) => println!("{}", queue.front().unwrap()),
            (3, _) => {
                queue.pop_front();
            }
            (_, _) => unreachable!(),
        }
    }
}
