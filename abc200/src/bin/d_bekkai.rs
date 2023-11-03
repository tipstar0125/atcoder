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

        let mut dp = vec![vec![]; 200];
        for (i, a) in A.iter().enumerate() {
            let mut ndp = dp.clone();
            for j in 0..200 {
                if dp[j].is_empty() {
                    continue;
                }
                let mut z = dp[j].clone();
                z.push(i + 1);
                if ndp[(j + a) % 200].is_empty() {
                    ndp[(j + a) % 200] = z;
                } else {
                    println!("Yes");
                    println!("{} {}", z.len(), z.iter().join(" "));
                    println!(
                        "{} {}",
                        ndp[(j + a) % 200].len(),
                        ndp[(j + a) % 200].iter().join(" ")
                    );
                    return;
                }
            }

            if ndp[a % 200].is_empty() {
                ndp[a % 200].push(i + 1);
            } else {
                println!("Yes");
                println!("1 {}", i + 1);
                println!("{} {}", ndp[a % 200].len(), ndp[a % 200].iter().join(" "));
                return;
            }
            dp = ndp;
        }
        println!("No");
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
