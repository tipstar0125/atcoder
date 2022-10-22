#![allow(non_snake_case)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        H:usize,
        W:usize,
        C: [Chars; H]
    }

    let mut ans = vec![];
    for j in 0..W {
        let mut sum = 0;
        for i in 0..H {
            if C[i][j] == '#' {
                sum += 1;
            }
        }
        ans.push(sum);
    }
    println!("{}", ans.iter().join(" ")); 
}
