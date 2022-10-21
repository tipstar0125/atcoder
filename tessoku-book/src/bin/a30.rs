#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        r: usize
    }

    let m = 1_000_000_007_usize;

    let mut a = 1_usize;
    let mut b = 1_usize;
    for i in 1..=n {
        a = (a * i) % m;
    }
    for i in 1..=r {
        b = (b * i) % m;
    }
    for i in 1..=(n - r) {
        b = (b * i) % m;
    }

    let b = power(b, m - 2, m);
    let ans = (a * b) % m;
    println!("{}", ans);
}

fn power(a: usize, b: usize, m: usize) -> usize {
    let mut p = a;
    let mut ans = 1;
    for i in 0..64 {
        if (b >> i) % 2 == 1 {
            ans = (ans * p) % m;
        }
        p = p * p % m;
    }
    ans
}
