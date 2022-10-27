#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
use std::collections::VecDeque;

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
        mut X: usize,
        mut Y: usize
    }

    let mut ans = VecDeque::new();
    ans.push_front((X, Y));

    while !(X == 1 && Y == 1) {
        if X > Y {
            X -= Y;
        } else {
            Y -= X;
        }
        ans.push_front((X, Y));
    }

    ans.pop_front();
    println!("{}", ans.len());
    for &a in &ans {
        let (x, y) = a;
        println!("{} {}", x, y);
    }
}
