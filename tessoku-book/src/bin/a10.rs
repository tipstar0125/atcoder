use proconio::{fastout, input};
use std::cmp;

#[allow(non_snake_case)]
#[fastout]

fn main() {
    input! {
        N: usize,
        A: [usize; N],
        D: usize,
        LR: [(usize, usize); D]
    }

    let mut P = vec![0; N + 1];
    let mut Q = vec![0; N + 1];

    for i in 1..=N {
        P[i] = cmp::max(P[i - 1], A[i - 1]);
    }
    for i in (1..=N).rev() {
        Q[i - 1] = cmp::max(Q[i], A[i - 1]);
    }

    for &(L, R) in &LR {
        let ans = cmp::max(P[L - 1], Q[R]);
        println!("{:?}", ans);
    }
}
