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
            T: Chars,
            S: [Chars; N]
        }

        let mut ans = vec![];
        for (i, s) in S.iter().enumerate() {
            if T.len() == s.len() {
                let mut cnt = 0;
                for j in 0..T.len() {
                    if T[j] != s[j] {
                        cnt += 1;
                    }
                }
                if cnt <= 1 {
                    ans.push(i + 1);
                }
            } else if T.len() == s.len() + 1 {
                let mut idx = 0;
                let mut ok = true;
                for j in 0..s.len() {
                    if T[j] != s[j] {
                        idx = j;
                        ok = false;
                        break;
                    }
                }
                if ok {
                    ans.push(i + 1);
                    continue;
                }
                ok = true;
                for j in 0..s.len() {
                    if j >= idx {
                        if T[j + 1] != s[j] {
                            ok = false;
                            break;
                        }
                    } else if T[j] != s[j] {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    ans.push(i + 1);
                }
            } else if T.len() + 1 == s.len() {
                let mut idx = 0;
                let mut ok = true;
                for j in 0..T.len() {
                    if T[j] != s[j] {
                        idx = j;
                        ok = false;
                        break;
                    }
                }
                if ok {
                    ans.push(i + 1);
                    continue;
                }
                ok = true;
                for j in 0..T.len() {
                    if j >= idx {
                        if T[j] != s[j + 1] {
                            ok = false;
                            break;
                        }
                    } else if T[j] != s[j] {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    ans.push(i + 1);
                }
            }
        }
        println!("{}", ans.len());
        println!("{}", ans.iter().join(" "));
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
