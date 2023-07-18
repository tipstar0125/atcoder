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
            T: usize,
            M: usize,
            AB: [(Usize1, Usize1); M]
        }

        let MAX = 1_usize << N;
        let mut hate_teams = vec![false; MAX];
        for &(a, b) in &AB {
            let mask = (1_usize << a) | (1_usize << b);
            for i in 0..MAX {
                if mask == (mask & i) {
                    hate_teams[i] = true;
                }
            }
        }

        let mut dp = vec![0_usize; MAX];
        dp[MAX - 1] = 1;
        for _ in 0..T {
            let mut ndp = vec![0_usize; MAX];
            for i in 1..MAX {
                let mut x = i;
                // x > 0 とすると、チームの順列分重複してカウントされる
                while x > (i ^ x) {
                    if !hate_teams[x] {
                        ndp[i ^ x] += dp[i];
                    }
                    // iを2進数で考えたときに、0の箇所が1ではない値に更新
                    x = (x - 1) & i;
                    // 以下のコメントアウトは上記と同じ更新
                    // x -= 1;
                    // while (i | x) > i {
                    //     x -= 1;
                    // }
                }
            }
            dp = ndp;
        }
        println!("{}", dp[0]);
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
