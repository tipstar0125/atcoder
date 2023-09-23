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
            X: usize,
            Y: usize,
            PT: [(usize, usize); N-1],
            Q: usize
        }

        let mut l = PT[0].0;
        for i in 1..N - 1 {
            let p = PT[i].0;
            l = lcm(l, p);
        }

        let mut times = vec![0; l];
        for s in 0..l {
            let mut now = s;
            for &(p, t) in &PT {
                if now % p != 0 {
                    now += p - now % p;
                }
                now += t;
            }
            times[s] = now - s;
        }

        for _ in 0..Q {
            input! {
                q: usize
            }
            let mut ans = q + X + Y;
            ans += times[(q + X) % l];
            println!("{}", ans);
        }
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

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
