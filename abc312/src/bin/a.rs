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
            S: String
        }

        let mut set = BTreeSet::new();
        set.insert("ACE".to_string());
        set.insert("BDF".to_string());
        set.insert("CEG".to_string());
        set.insert("DFA".to_string());
        set.insert("EGB".to_string());
        set.insert("FAC".to_string());
        set.insert("GBD".to_string());

        if set.contains(&S) {
            println!("Yes");
        } else {
            println!("No");
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
