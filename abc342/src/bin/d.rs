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
            A: [usize; N]
        }

        let e = Eratosthenes::new(2e5 as usize);
        let mut zero = 0;
        let mut mp: BTreeMap<Vec<usize>, usize> = BTreeMap::new();
        for &a in &A {
            if a == 0 {
                zero += 1;
                continue;
            }
            let f = e.factorize(a);
            let mut v = vec![];
            for &(k, x) in f.iter() {
                if x % 2 == 1 {
                    v.push(k);
                }
            }
            *mp.entry(v).or_default() += 1;
        }
        let mut ans = zero * (N - zero);
        if zero > 1 {
            ans += zero * (zero - 1) / 2;
        }
        for (_, v) in mp.iter() {
            if *v > 1 {
                ans += v * (v - 1) / 2;
            }
        }
        println!("{}", ans);
    }
}

#[derive(Debug, Clone, Default)]
struct Eratosthenes {
    is_prime: Vec<bool>,
    min_factor: Vec<isize>,
    mobius: Vec<isize>,
}

impl Eratosthenes {
    fn new(n: usize) -> Self {
        let mut is_prime = vec![true; n + 1];
        let mut min_factor = vec![-1; n + 1];
        let mut mobius = vec![1; n + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        min_factor[1] = 1;

        for p in 2..=n {
            if !is_prime[p] {
                continue;
            }
            min_factor[p] = p as isize;
            mobius[p] = -1;

            let mut q = p * 2;
            while q <= n {
                is_prime[q] = false;
                if min_factor[q] == -1 {
                    min_factor[q] = p as isize;
                }
                if (q / p) % p == 0 {
                    mobius[q] = 0;
                } else {
                    mobius[q] = -mobius[q];
                }
                q += p;
            }
        }
        Eratosthenes {
            is_prime,
            min_factor,
            mobius,
        }
    }
    fn factorize(&self, N: usize) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        let mut n = N;
        while n > 1 {
            let p = self.min_factor[n];
            let mut exp = 0;

            while self.min_factor[n] == p {
                n /= p as usize;
                exp += 1;
            }
            ret.push((p as usize, exp));
        }
        ret
    }
    fn dividers(&self, N: usize) -> Vec<usize> {
        let mut ret = vec![1];
        let pf = self.factorize(N);
        for &(p, exp) in &pf {
            let m = ret.len();
            for i in 0..m {
                let mut v = 1;
                for _ in 0..exp {
                    v *= p;
                    ret.push(ret[i] * v);
                }
            }
        }
        ret.sort();
        ret
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
