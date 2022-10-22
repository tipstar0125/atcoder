#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        A: f64,
        B: f64
    }
    
    println!("{:.3}", B/A);
}
