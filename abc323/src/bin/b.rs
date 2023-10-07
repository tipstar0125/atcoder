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
            S: [Chars; N]
        }

        let mut cnt = vec![0; N];
        for i in 0..N {
            for &c in &S[i] {
                if c == 'o' {
                    cnt[i] += 1;
                }
            }
        }
        let mut a = vec![];
        for i in 0..N {
            a.push((cnt[i], i));
        }
        a.sort_by(|a, b| (a.0, Reverse(a.1)).cmp(&(b.0, Reverse(b.1))));
        a.reverse();
        let mut ans = vec![];
        for i in 0..N {
            ans.push(a[i].1 + 1);
        }
        println!("{}", ans.iter().join(" "));
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
