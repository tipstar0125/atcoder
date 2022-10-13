use itertools::Itertools;
use proconio::{fastout, input};

#[allow(non_snake_case)]
#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(usize, usize); M]
    }

    let mut G = vec![vec![]; N + 1];

    for &(A, B) in &AB {
        G[A].push(B);
        G[B].push(A);
    }

    for i in 1..=N {
        let ans = G[i].iter().join(", ");
        println!("{:?}: {{{}}}", i, ans);
    }
}
