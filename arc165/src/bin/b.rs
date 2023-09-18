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
            P: [usize; N]
        }

        // Same
        let mut A = vec![0];
        let mut now = 0;
        for i in 1..N {
            if P[i - 1] < P[i] {
                now += 1;
            }
            A.push(now);
        }

        let mut same = false;
        for i in 0..N - K + 1 {
            if A[i + K - 1] - A[i] == K - 1 {
                same = true;
            }
        }
        if same {
            println!("{}", P.iter().join(" "));
            return;
        }

        // 最後のK項をソートしたものが最低条件
        // よって、N-K項より前はそのままが最低条件
        // N-K項より前だけをソートするケースはNG
        // N-K項を跨いで、それ以降をソートするケースを考える
        // 1. P_iからP_N-Kは昇順（N-K項より前がソートされない条件）
        // 2. P_N-K+1からP_i+K-1の最小がP_N-Kより大きい（N-K項より前に数字がこない条件）
        // iが小さい方が、ソートされる範囲が少ないので、iが小さい方が良い

        let mut min = N + 1;
        let mut S_min = vec![min; N - K];
        for i in N - K..N {
            if P[i] < min {
                min = P[i];
            }
            S_min.push(min);
        }

        let mut idx = N - K;

        for i in 0..N - K {
            if i + K - 1 < N - K {
                continue;
            }
            if A[N - K - 1] - A[i] == N - K - 1 - i && P[N - K - 1] < S_min[i + K - 1] {
                idx = i;
                break;
            }
        }

        let mut ans = P[0..idx].iter().collect_vec();
        let mut v = P[idx..idx + K].iter().collect_vec();
        v.sort();
        ans.extend(v);
        ans.extend(P[idx + K..N].iter().collect_vec());
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
