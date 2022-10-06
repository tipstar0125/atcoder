use std::collections::VecDeque;

use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        mut a: [usize; N]
    }
    a.sort();
    a.dedup();
    while a.len() < N {
        a.push(0);
    }

    let mut b: VecDeque<_> = a.iter().cloned().collect();
    let mut ans = 0;

    while !b.is_empty() {
        if b[0] == ans + 1 {
            ans += 1;
            b.pop_front();
            continue;
        }

        if b.len() >= 2 {
            b.pop_back();
            b.pop_back();
            ans += 1;
        } else {
            break;
        }
    }

    println!("{:?}", ans);
}
