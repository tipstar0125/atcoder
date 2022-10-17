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
        S: Chars
    }

    let mut count = vec![1; N];

    for (i, &s) in S.iter().enumerate() {
        if s == 'A' {
            count[i + 1] = count[i] + 1;
        }
    }
    for (i, &s) in S.iter().enumerate().rev() {
        if s == 'B' {
            count[i] = count[i].max(count[i + 1] + 1);
        }
    }
    println!("{}", count.iter().sum::<usize>());
}
