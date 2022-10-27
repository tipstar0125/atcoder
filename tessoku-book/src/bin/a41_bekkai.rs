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
        N: usize,
        S: Chars
    }

    let mut ans = "No";
    for i in 0..N - 2 {
        if S[i] == S[i + 1] && S[i + 1] == S[i + 2] {
            ans = "Yes";
        }
    }
    println!("{}", ans);
}
