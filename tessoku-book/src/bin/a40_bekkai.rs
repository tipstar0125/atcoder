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
        mut A: [usize; N]
    }

    A.sort();
    A.push(101);
    let mut length = 0;
    let mut n = 0_usize;
    let mut ans = 0;
    for &a in &A {
        if length != a {
            if n >= 3 {
                ans += n * (n - 1) * (n - 2) / 6;
            }
            length = a;
            n = 1;
        } else {
            n += 1;
        }
    }
    println!("{}", ans);
}
