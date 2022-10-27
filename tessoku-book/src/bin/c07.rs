#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::comparison_chain)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use superslice::Ext;

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
        mut C: [usize; N],
        Q: usize,
        X: [usize; Q]
    }

    C.sort();
    let mut S = vec![0; N + 1];
    for (i, &c) in C.iter().enumerate() {
        S[i + 1] = S[i] + c;
    }

    for x in &X {
        let idx = S.upper_bound(x);
        println!("{}", idx - 1);
    }
}
