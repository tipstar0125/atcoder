use std::collections::HashSet;

use proconio::{fastout, input};

#[allow(non_snake_case)]
#[allow(clippy::needless_range_loop)]
#[fastout]

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
        B: [usize; N],
        C: [usize; N],
        D: [usize; N]
    }

    let mut P = HashSet::new();
    let mut Q = HashSet::new();

    for i in 0..N {
        for j in 0..N {
            P.insert(K - A[i] - B[j]);
            Q.insert(C[i] + D[j]);
        }
    }

    let result = &P & &Q;
    if !result.is_empty() {
        println!("Yes");
    } else {
        println!("No");
    }
}
