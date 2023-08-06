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
    fn solve(&mut self) {
        let mut stdin =
            proconio::source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));
        macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
        input! {
            N: usize,
            K: usize,
        }

        if K == 1 {
            let mut ans = vec![];
            for i in 0..N {
                println!("? {}", i + 1);
                input! {
                    t: usize
                }
                if t == 0 {
                    ans.push(0);
                } else {
                    ans.push(1);
                }
            }
            println!("! {}", ans.iter().join(" "));
            return;
        }

        let mut b = vec![false; N];
        b[0] = true;

        let q = (1..=K).collect_vec();
        println!("? {}", q.iter().join(" "));
        input! {
            T0: usize
        }

        let mut now = K + 1;
        while now <= N {
            let mut q = (2..=K).collect_vec();
            q.push(now);
            println!("? {}", q.iter().join(" "));
            input! {
                t: usize
            }
            if t == T0 {
                b[now - 1] = true;
            }
            now += 1;
        }

        for x in 2..=K {
            let mut q: BTreeSet<_> = (1..=K + 1).collect();
            q.remove(&x);
            println!("? {}", q.iter().join(" "));
            input! {
                t: usize
            }
            if t == T0 {
                if b[K] {
                    b[x - 1] = true;
                }
            } else if !b[K] {
                b[x - 1] = true;
            }
        }

        let cnt = b[..K].iter().filter(|&&b| b).count();
        let mut ans = vec![];
        if (cnt % 2 == 0 && T0 == 0) || (cnt % 2 != 0 && T0 != 0) {
            for bi in b {
                if bi {
                    ans.push(1);
                } else {
                    ans.push(0);
                }
            }
        } else {
            for bi in b {
                if bi {
                    ans.push(0);
                } else {
                    ans.push(1);
                }
            }
        }
        println!("! {}", ans.iter().join(" "));
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
