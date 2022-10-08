use proconio::{fastout, input};

#[allow(non_snake_case)]
#[allow(clippy::needless_range_loop)]
#[fastout]

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N]
    }

    let mut l = 0;
    let mut r = 0;
    let mut ans = 0;

    while l < N - 1 {
        if r + 1 == N || A[r + 1] - A[l] > K {
            ans += r - l;
            l += 1;
        } else {
            r += 1;
        }
        println!("{:?}", r);
    }
    println!("{:?}", ans);
}
