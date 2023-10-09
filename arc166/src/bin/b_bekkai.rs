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
            a: usize,
            b: usize,
            c: usize,
            D: [usize; N]
        }
        let lcm_ab = lcm(a, b);
        let lcm_bc = lcm(b, c);
        let lcm_ca = lcm(c, a);
        let lcm_abc = lcm(lcm_ab, c);

        fn calc_cost(x: usize, y: usize) -> usize {
            x * ((y + x - 1) / x) - y
        }

        let INF = 1_usize << 60;
        let mut dp = vec![vec![INF; 8]; N + 1];
        dp[0][0] = 0;
        for i in 1..=N {
            let d = D[i - 1];
            let mut cost = vec![0; 8];
            cost[1] = calc_cost(a, d);
            cost[2] = calc_cost(b, d);
            cost[4] = calc_cost(c, d);
            cost[3] = calc_cost(lcm_ab, d);
            cost[5] = calc_cost(lcm_ca, d);
            cost[6] = calc_cost(lcm_bc, d);
            cost[7] = calc_cost(lcm_abc, d);

            for j in 0..8 {
                dp[i][j] = min!(dp[i][j], dp[i - 1][j]);
                for k in 0..8 {
                    dp[i][j | k] = min!(dp[i][j | k], dp[i - 1][j] + cost[k]);
                }
            }
        }
        let ans = dp[N][7];
        println!("{}", ans);
    }
}

fn ext_gcd(a: usize, b: usize) -> (isize, isize, usize) {
    if a == 0 {
        return (0, 1, b);
    }
    let (x, y, g) = ext_gcd(b % a, a);
    (y - b as isize / a as isize * x, x, g)
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / ext_gcd(a, b).2
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
