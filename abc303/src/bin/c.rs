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
use superslice::Ext;

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
            _N: usize,
            M: usize,
            mut H: isize,
            K: isize,
            S: Chars,
            XY: [(isize, isize); M]
        }

        let mut items = BTreeSet::new();
        for &(x, y) in &XY {
            items.insert((x, y));
        }

        let mut x = 0_isize;
        let mut y = 0_isize;

        for s in S {
            if s == 'R' {
                x += 1;
            } else if s == 'L' {
                x -= 1;
            } else if s == 'U' {
                y += 1;
            } else {
                y -= 1;
            }
            H -= 1;
            if H < 0 {
                println!("No");
                return;
            }
            if items.contains(&(x, y)) && H < K {
                H = K;
                items.remove(&(x, y));
            }
        }
        println!("Yes");
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
