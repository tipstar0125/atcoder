#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        N: usize
    }

    let mut ans = "".to_string();
    let lucky = "47";
    for i in (0..10).rev() {
        let index = (N - 1) / (1 << i) % 2;
        ans += &lucky[index..index + 1];
    }
    println!("{}", ans);
}
