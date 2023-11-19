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
            XY: [(isize, isize); N]
        }

        if K == 1 {
            println!("Infinity");
            return;
        }

        let mut mp: BTreeMap<(isize, isize), BTreeSet<usize>> = BTreeMap::new();
        for i in 0..N {
            for j in i + 1..N {
                let (x0, y0) = XY[i];
                let (x1, y1) = XY[j];
                let mut dx = x0 - x1;
                let mut dy = y0 - y1;
                let dx_abs = dx.unsigned_abs();
                let dy_abs = dy.unsigned_abs();
                let (_, _, g) = ext_gcd(dx_abs, dy_abs);
                dx /= g as isize;
                dy /= g as isize;
                if dx < 0 {
                    dx *= -1;
                    dy *= -1;
                }
                mp.entry((dx, dy)).or_default().insert(i);
                mp.entry((dx, dy)).or_default().insert(j);
            }
        }
        let mut ans = 0_usize;
        println!("{:?}", mp);
        for (_, v) in mp.iter() {
            if v.len() < K {
                continue;
            }
            println!("{} {} {}", v.len(), K, nCr(v.len(), K));
            ans += nCr(v.len(), K);
        }
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

fn nCr(n: usize, r: usize) -> usize {
    let mut ret = 1;
    for i in 1..=r {
        ret *= n - i + 1;
        ret /= i;
    }
    ret
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
