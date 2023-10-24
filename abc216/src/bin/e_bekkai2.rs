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
            mut K: usize,
            mut A: [usize; N]
        }

        A.sort();

        let mut m = A.pop().unwrap();
        let mut cnt = 1;
        let mut ans = 0_usize;

        while K > 0 && m > 0 {
            if !A.is_empty() {
                let next = *A.last().unwrap();
                if m == next {
                    cnt += 1;
                    A.pop();
                    continue;
                }

                let d = m - next;
                let s = (d * m - d * (d - 1) / 2) * cnt;
                let c = d * cnt;
                if K > c {
                    K -= c;
                    ans += s;
                    m = next;
                } else {
                    let d = K / cnt;
                    let s = (d * m - d * (d - 1) / 2) * cnt;
                    let c = d * cnt;
                    K -= c;
                    ans += s;
                    m -= d;
                    ans += m * (K % cnt);
                    K = 0;
                }
            } else {
                let d = K / cnt;
                if d * m >= d * (d - 1) / 2 {
                    let s = (d * m - d * (d - 1) / 2) * cnt;
                    let c = d * cnt;
                    K -= c;
                    ans += s;
                    m -= d;
                    ans += m * (K % cnt);
                    K = 0;
                } else {
                    let d = m;
                    let s = (d * m - d * (d - 1) / 2) * cnt;
                    ans += s;
                    m = 0;
                }
            }
        }
        println!("{}", ans);
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
