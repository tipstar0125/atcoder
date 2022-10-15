#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let mut B = A.clone();
    B.sort();
    B.dedup();

    let map: BTreeMap<_, _> = B.iter().enumerate().map(|(i, x)| (x, i + 1)).collect();
    let mut ans = vec![0; N];

    for (i, a) in A.iter().enumerate() {
        ans[i] = map[a];
    }
    println!("{}", ans.iter().join(" "));
}
