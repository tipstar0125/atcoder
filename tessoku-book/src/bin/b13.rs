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

    let mut S = vec![0; N + 1];
    for i in 0..N {
        S[i + 1] = A[i] + S[i];
    }

    let mut left = 0;
    let mut right = 0;
    let mut ans = 0;

    while left < N {
        if right < N && S[right + 1] - S[left] <= K {
            right += 1;
        } else {
            ans += right - left;
            left += 1;
        }
    }

    println!("{:?}", ans);
}
