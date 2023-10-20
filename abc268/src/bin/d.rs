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
            M: usize,
            S: [Chars; N],
            T: [String; M]
        }

        let mut set = BTreeSet::new();
        for t in T {
            set.insert(t);
        }

        let mut sum = 0_usize;
        for s in S.iter() {
            sum += s.len();
        }
        for s in S.iter().permutations(N) {
            let m = 16 - sum - (N - 1);
            let max = (m + 1).pow(N as u32 - 1);
            for i in 0..max {
                let mut v = vec![0];
                let mut n = i;
                for _ in 0..N - 1 {
                    if n == 0 {
                        v.push(0);
                        continue;
                    }
                    v.push(n % m);
                    n /= m;
                }

                let ss = v.iter().sum::<usize>();
                if ss > m {
                    continue;
                }

                let mut name = s[0].iter().join("");
                for i in 1..N {
                    for _ in 0..v[i] + 1 {
                        name += "_";
                    }
                    name += s[i].iter().join("").as_str();
                }
                if !set.contains(&name) && name.len() >= 3 {
                    println!("{}", name);
                    return;
                }
            }
        }
        println!("-1");
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
