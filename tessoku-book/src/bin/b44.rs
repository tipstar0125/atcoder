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
        A: [[usize; N]; N],
        Q: usize,
        query: [(usize, usize, usize); Q]
    }

    let mut T: Vec<usize> = (0..N).collect();
    for &q in &query {
        match q {
            (1, x, y) => {
                T.swap(x - 1, y - 1);
            }
            (2, x, y) => {
                println!("{}", A[T[x - 1]][y - 1]);
            }
            (_, _, _) => unreachable!(),
        }
    }
}
