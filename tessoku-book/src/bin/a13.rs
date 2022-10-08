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

    let mut right = 0;
    let mut ans = 0;
    for left in 0..N - 1 {
        while right + 1 < N && A[right + 1] - A[left] <= K {
            right += 1;
        }
        ans += right - left;
    }

    println!("{:?}", ans);
}
