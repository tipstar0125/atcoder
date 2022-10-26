#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

pub mod macros {
    #[macro_export]
    macro_rules !max {
        ($x: expr) => ($x);
        ($x: expr, $($y: expr), +) => {
            std::cmp::max($x, max!($($y), +))
        }
    }
    #[macro_export]
    macro_rules !min {
        ($x: expr) => ($x);
        ($x: expr, $($y: expr), +) => {
            std::cmp::min($x, min!($($y), +))
        }
    }
}

#[fastout]
fn main() {
    input! {
        N:usize,
        M:usize,
        AB: [(usize, usize); M]
    }

    let mut G = vec![vec![]; N + 1];
    for &(A, B) in &AB {
        G[A].push(B);
        G[B].push(A);
    }
    let mut dist = vec![-1; N + 1];
    let mut Q: Vec<usize> = vec![1];
    dist[1] = 0;

    for i in 1..N {
        let mut next = vec![];
        for &q in &Q {
            for &pos in &G[q] {
                if dist[pos] == -1 {
                    next.push(pos);
                    dist[pos] = i as isize;
                }
            }
        }
        Q = next;
    }

    for a in &dist[1..] {
        println!("{}", a);
    }
}
