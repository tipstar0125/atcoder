use num::integer::gcd;
use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        mut A: usize,
        mut B: usize,
    }

    let gcd = gcd(A, B);
    let ans = A * B / gcd;
    println!("{:?}", ans);
}
