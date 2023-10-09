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

        let mut cost_a = vec![];
        let mut cost_b = vec![];
        let mut cost_c = vec![];
        let mut cost_ab = vec![];
        let mut cost_bc = vec![];
        let mut cost_ca = vec![];
        let mut cost_abc = vec![];
        for (i, &d) in D.iter().enumerate() {
            cost_a.push((calc_cost(a, d), i));
            cost_b.push((calc_cost(b, d), i));
            cost_c.push((calc_cost(c, d), i));
            cost_ab.push((calc_cost(lcm_ab, d), i));
            cost_bc.push((calc_cost(lcm_bc, d), i));
            cost_ca.push((calc_cost(lcm_ca, d), i));
            cost_abc.push((calc_cost(lcm_abc, d), i));
        }
        let INF = 1_usize << 60;
        let mut ans = INF;

        cost_a.sort();
        cost_b.sort();
        cost_c.sort();
        cost_ab.sort();
        cost_bc.sort();
        cost_ca.sort();
        cost_abc.sort();

        let r = min!(N, 3);
        let cost_a_top = cost_a[0..r].iter().collect_vec();
        let cost_b_top = cost_b[0..r].iter().collect_vec();
        let cost_c_top = cost_c[0..r].iter().collect_vec();
        let cost_ab_top = cost_ab[0..r].iter().collect_vec();
        let cost_bc_top = cost_bc[0..r].iter().collect_vec();
        let cost_ca_top = cost_ca[0..r].iter().collect_vec();
        let cost_abc_top = cost_abc[0..r].iter().collect_vec();

        for i in 0..r {
            for j in 0..r {
                for k in 0..r {
                    let (ca, idx_a) = cost_a_top[i];
                    let (cb, idx_b) = cost_b_top[j];
                    let (cc, idx_c) = cost_c_top[k];
                    if idx_a != idx_b && idx_b != idx_c && idx_c != idx_a {
                        ans = min!(ans, ca + cb + cc);
                    }
                }
            }
        }
        for i in 0..r {
            for j in 0..r {
                let (ca, idx_a) = cost_a_top[i];
                let (cbc, idx_bc) = cost_bc_top[j];
                if idx_a != idx_bc {
                    ans = min!(ans, ca + cbc);
                }
            }
        }
        for i in 0..r {
            for j in 0..r {
                let (cb, idx_b) = cost_b_top[i];
                let (cca, idx_ca) = cost_ca_top[j];
                if idx_b != idx_ca {
                    ans = min!(ans, cb + cca);
                }
            }
        }
        for i in 0..r {
            for j in 0..r {
                let (cc, idx_c) = cost_c_top[i];
                let (cab, idx_ab) = cost_ab_top[j];
                if idx_c != idx_ab {
                    ans = min!(ans, cc + cab);
                }
            }
        }
        ans = min!(ans, cost_abc_top[0].0);
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
