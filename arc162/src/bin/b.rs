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
            mut P: [Usize1; N]
        }

        let mut sorted_P = P.clone();
        let mut inverse_num = 0_usize;
        for _ in 0..N {
            for i in 1..N {
                if sorted_P[i - 1] > sorted_P[i] {
                    sorted_P.swap(i - 1, i);
                    inverse_num += 1;
                }
            }
        }

        if inverse_num % 2 == 1 {
            println!("No");
            return;
        }

        let mut now = 0;
        let mut ans = vec![];
        let mut i = 0;
        let mut max = 2000;
        while i < max {
            if P[now] == now {
                now += 1;
                max += 1;
            } else {
                let mut pos = now;
                for i in now..N {
                    if P[i] == now {
                        pos = i;
                    }
                }
                if pos == N - 1 {
                    pos -= 1;
                }
                let A = &P[..now].to_vec();
                let B = &P[pos..pos + 2].to_vec();
                let C = &P[now..pos].to_vec();
                let D = &P[pos + 2..].to_vec();
                let mut PP = A.clone();
                PP.extend(B.clone());
                PP.extend(C.clone());
                PP.extend(D.clone());
                P = PP;
                ans.push((pos, now));
            }
            if P == sorted_P {
                break;
            }
            i += 1;
        }
        println!("Yes");
        println!("{}", ans.len());
        for &(pos, now) in &ans {
            println!("{} {}", pos + 1, now);
        }
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
