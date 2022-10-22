#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        N:usize,
        M: usize,
        mut T: usize,
        A: [usize; N-1],
        XY: [(usize, usize); M]
    }

    let mut bonus = vec![0; N + 1];
    for &(x, y) in &XY {
        bonus[x] += y;
    }
    let mut goal = 1;

    for (i, &a) in A.iter().enumerate() {
        T += bonus[i + 1];
        if T <= a {
            break;
        } else {
            T -= a;
            goal += 1;
        }
    }

    if goal == N {
        println!("Yes");
    } else {
        println!("No");
    }
}
