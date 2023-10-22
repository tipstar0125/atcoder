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
            K: usize,
            P: [Usize1; N]
        }

        let mut ans = vec![-1; N];
        let mut min_vec = vec![];
        let mut vv = vec![];
        for (i, &p) in P.iter().enumerate() {
            let idx = min_vec.lower_bound(&p);
            if idx == min_vec.len() {
                min_vec.push(p);
                vv.push(vec![p]);
            } else {
                min_vec[idx] = p;
                vv[idx].push(p);
            }
            if vv[idx].len() == K {
                for j in &vv[idx] {
                    ans[*j] = i as isize + 1;
                }
                min_vec.remove(idx);
                vv.remove(idx);
            }
        }
        println!("{}", ans.iter().join("\n"));
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
