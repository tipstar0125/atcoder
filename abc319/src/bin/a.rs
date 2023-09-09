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
        // tourist 3858
        // ksun48 3679
        // Benq 3658
        // Um_nik 3648
        // apiad 3638
        // Stonefeang 3630
        // ecnerwala 3613
        // mnbvmar 3555
        // newbiedmy 3516
        // semiexp 3481
        let mut mp = BTreeMap::new();
        mp.insert("tourist".to_string(), 3858);
        mp.insert("ksun48".to_string(), 3679);
        mp.insert("Benq".to_string(), 3658);
        mp.insert("Um_nik".to_string(), 3648);
        mp.insert("apiad".to_string(), 3638);
        mp.insert("Stonefeang".to_string(), 3630);
        mp.insert("ecnerwala".to_string(), 3613);
        mp.insert("mnbvmar".to_string(), 3555);
        mp.insert("newbiedmy".to_string(), 3516);
        mp.insert("semiexp".to_string(), 3481);
        println!("{}", mp[&S]);
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
