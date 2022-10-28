#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::comparison_chain)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

pub mod macros {
    #[macro_export]
    macro_rules! max {
        ($x: expr) => ($x);
        ($x: expr, $( $y: expr ),+) => {
            std::cmp::max($x, max!($( $y ),+))
        }
    }
    #[macro_export]
    macro_rules! min {
        ($x: expr) => ($x);
        ($x: expr, $( $y: expr ),+) => {
            std::cmp::min($x, min!($( $y ),+))
        }
    }
}

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        Query: [(usize, usize, usize); Q]
    }

    let mut parent = vec![0; N + 1];

    for &q in &Query {
        match q {
            (1, u, v) => {
                if parent[u] == 0 && parent[v] == 0 {
                    parent[u] = u;
                    parent[v] = u;
                } else if parent[u] != 0 {
                    if parent[v] == 0 {
                        parent[v] = parent[u];
                    } else {
                        let root = parent[v];
                        for i in 1..=N {
                            if parent[i] == root {
                                parent[i] = parent[u];
                            }
                        }
                    }
                } else {
                    parent[u] = parent[v];
                }
            }
            (2, u, v) => {
                if (parent[u] == parent[v]) && parent[u] != 0 {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            (_, _, _) => unreachable!(),
        }
    }
}
