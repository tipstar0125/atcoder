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
            N: usize
        }

        let mut cnt: Vec<BTreeMap<usize, usize>> = vec![BTreeMap::new(); N + 1];
        let mut deleted = vec![false; N + 1];
        for i in 2..=N {
            if deleted[i] {
                continue;
            }
            let mut now = i;
            while now <= N {
                let mut c = 0;
                let mut now2 = now;
                while now2 % i == 0 {
                    c += 1;
                    now2 /= i;
                }
                *cnt[now].entry(i).or_default() ^= c % 2;
                if cnt[now][&i] == 0 {
                    cnt[now].remove(&i);
                }
                deleted[now] = true;
                now += i;
            }
        }

        let mut mp: BTreeMap<BTreeMap<usize, usize>, usize> = BTreeMap::new();
        for i in 1..=N {
            *mp.entry(cnt[i].clone()).or_default() += 1;
        }
        let mut ans = 0;
        for (_, x) in mp.iter() {
            ans += x * x;
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
