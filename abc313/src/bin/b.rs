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
            N: usize,
            M: usize,
            AB: [(Usize1, Usize1); M]
        }

        let mut outdegs = vec![vec![]; N];
        for &(a, b) in &AB {
            outdegs[b].push(a);
        }
        let cnt = outdegs.iter().filter(|&v| v.is_empty()).count();
        if cnt == 1 {
            for (i, d) in outdegs.iter().enumerate() {
                if d.is_empty() {
                    println!("{}", i + 1);
                    return;
                }
            }
        } else {
            println!("-1");
        }
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
