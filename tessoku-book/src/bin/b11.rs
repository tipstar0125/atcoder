use proconio::{fastout, input};
use superslice::Ext;

#[allow(non_snake_case)]
#[fastout]

fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
        Q: usize,
        X: [usize; Q]
    }

    A.sort();

    for x in &X {
        println!("{:?}", A.lower_bound(x));
    }
}
