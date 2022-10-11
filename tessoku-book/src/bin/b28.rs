use std::vec;

use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize
    }

    let mut a = vec![1; N];

    for i in 2..N {
        a[i] = (a[i - 1] + a[i - 2]) % 1000000007;
    }

    println!("{:?}", a[N - 1]);
}
