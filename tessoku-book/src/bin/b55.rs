#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::{collections::BTreeSet, ops::BitOrAssign};

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        Q: usize,
        Query: [(usize, isize); Q]
    }

    let mut set = BTreeSet::new();
    for q in &Query {
        match q {
            (1, x) => {
                set.insert(*x);
            }
            (2, x) => {
                if set.is_empty() {
                    println!("-1");
                } else {
                    let first = set.iter().next().unwrap();
                    let value1 = (set.range(x..).next().unwrap_or(first) - x).abs();
                    let value2 = (set.range(..x).next_back().unwrap_or(first) - x).abs();
                    if value1 < value2 {
                        println!("{}", value1);
                    } else {
                        println!("{}", value2);
                    }
                }
            }
            (_, _) => unreachable!(),
        }
    }
}
