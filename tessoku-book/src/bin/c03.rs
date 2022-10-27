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
        D: usize,
        X: usize,
        A: [isize; D-1],
        Q: usize,
        ST: [(usize, usize); Q]
    }
    let mut S = vec![0_isize; D + 1];
    S[1] = X as isize;
    for (i, &a) in A.iter().enumerate() {
        S[i + 2] = S[i + 1] + a;
    }

    for &(s, t) in &ST {
        if S[s] > S[t] {
            println!("{}", s);
        } else if S[s] < S[t] {
            println!("{}", t);
        } else {
            println!("Same");
        }
    }
}
