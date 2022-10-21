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
        Q: usize
    }

    let mut query = vec![];
    for _ in 0..Q {
        input! {
            n: usize
        }
        if n == 1 {
            input! {
                x: usize,
                y: usize
            }
            query.push((n, x, y));
        } else if n == 2 {
            query.push((n, 0, 0));
        } else {
            input! {
                x: usize,
            }
            query.push((n, x, 0));
        }
    }

    let mut A = vec![];
    for i in 1..=N {
        A.push(i);
    }
    let mut is_reversed = false;

    for &q in &query {
        match q {
            (1, x, y) => {
                if is_reversed {
                    A[N - x] = y;
                } else {
                    A[x - 1] = y;
                }
            }
            (2, _, _) => {
                is_reversed = !is_reversed;
            }
            (3, x, _) => {
                if is_reversed {
                    println!("{}", A[N - x]);
                } else {
                    println!("{}", A[x - 1]);
                }
            }
            (_, _, _) => unreachable!(),
        }
    }
}
