#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            T: usize
        }

        for _ in 0..T {
            input! {
                N: usize,
                mut X: usize,
                mut K: usize,
            }

            if K == 0 {
                println!("1");
                continue;
            }

            let mut ans = count(X, K, N);

            while X != 1 && K != 1 {
                ans += count(X ^ 1, K - 2, N);
                X >>= 1;
                K -= 1;
            }
            if X > 1 {
                ans += 1;
            }

            println!("{}", ans);
        }
    }
}

fn count(now: usize, d: usize, N: usize) -> usize {
    let mut l = now;
    let mut r = now;
    let m = min!(d, now.leading_zeros() as usize);
    l <<= m;
    r <<= m;
    r += (1_usize << m) - 1;
    if l > N {
        return 0;
    }
    if r <= N {
        return r - l + 1;
    } else {
        return N - l + 1;
    }
}

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

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
