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

    let mut left = 0;
    let mut right = 0;
    let mut ans = 0;

    while left < N - 1 {
        if right + 1 < N && A[right + 1] - A[left] <= K {
            right += 1;
        } else {
            ans += right - left;
            left += 1;
        }
    }
    println!("{:?}", ans);
}
