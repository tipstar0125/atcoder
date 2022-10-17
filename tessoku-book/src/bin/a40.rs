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
        A: [usize; N]
    }

    let mut count = [0; 101];
    let mut ans = 0_isize;
    for &a in &A {
        count[a] += 1;
    }

    for i in 1..=100 {
        let n = count[i];
        ans += n * (n - 1) * (n - 2) / 6;
    }

    println!("{}", ans);
}
