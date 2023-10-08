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

        let INF = 1_usize << 60;
        let mut dp = vec![vec![INF; 8]; N + 1];
        dp[0][0] = 0;
        for i in 1..=N {
            let d = D[i - 1];
            let cost_a = a * ((d + a - 1) / a) - d;
            let cost_b = b * ((d + b - 1) / b) - d;
            let cost_c = c * ((d + c - 1) / c) - d;
            let cost_lcm_ab = lcm_ab * ((d + lcm_ab - 1) / lcm_ab) - d;
            let cost_lcm_bc = lcm_bc * ((d + lcm_bc - 1) / lcm_bc) - d;
            let cost_lcm_ca = lcm_ca * ((d + lcm_ca - 1) / lcm_ca) - d;
            let cost_lcm_abc = lcm_abc * ((d + lcm_abc - 1) / lcm_abc) - d;
            for j in 0..8 {
                dp[i][j] = dp[i - 1][j];
            }
            dp[i][1] = min!(dp[i][1], dp[i - 1][0] + cost_a);
            dp[i][2] = min!(dp[i][2], dp[i - 1][0] + cost_b);
            dp[i][3] = min!(dp[i][3], dp[i - 1][0] + cost_c);

            dp[i][4] = min!(dp[i][4], dp[i - 1][0] + cost_lcm_ab);
            dp[i][4] = min!(dp[i][4], dp[i - 1][1] + cost_b);
            dp[i][4] = min!(dp[i][4], dp[i - 1][2] + cost_a);

            dp[i][5] = min!(dp[i][5], dp[i - 1][0] + cost_lcm_bc);
            dp[i][5] = min!(dp[i][5], dp[i - 1][2] + cost_c);
            dp[i][5] = min!(dp[i][5], dp[i - 1][3] + cost_b);

            dp[i][6] = min!(dp[i][6], dp[i - 1][0] + cost_lcm_ca);
            dp[i][6] = min!(dp[i][6], dp[i - 1][1] + cost_c);
            dp[i][6] = min!(dp[i][6], dp[i - 1][3] + cost_a);

            dp[i][7] = min!(dp[i][7], dp[i - 1][0] + cost_lcm_abc);
            dp[i][7] = min!(dp[i][7], dp[i - 1][5] + cost_a);
            dp[i][7] = min!(dp[i][7], dp[i - 1][6] + cost_b);
            dp[i][7] = min!(dp[i][7], dp[i - 1][4] + cost_c);
            dp[i][7] = min!(dp[i][7], dp[i - 1][3] + cost_lcm_ab);
            dp[i][7] = min!(dp[i][7], dp[i - 1][1] + cost_lcm_bc);
            dp[i][7] = min!(dp[i][7], dp[i - 1][2] + cost_lcm_ca);
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
