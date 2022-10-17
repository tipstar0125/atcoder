#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        N: usize,
        mut LR: [(usize, usize); N]
    }

    let mut current_time = 0;
    let mut ans = 0;
    LR.sort_by(|a, b| a.1.cmp(&b.1));

    for &(l, r) in &LR {
        if current_time <= l {
            current_time = r;
            ans += 1;
        }
    }

    println!("{}", ans);
}
