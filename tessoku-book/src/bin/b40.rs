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

    let mut count = [0; 100];
    for &a in &A {
        count[a % 100] += 1;
    }

    let mut ans: isize = count[0] * (count[0] - 1) / 2;
    ans += count[50] * (count[50] - 1) / 2;

    for i in 1..=49 {
        ans += count[i] * count[100 - i]
    }

    println!("{}", ans);
}
