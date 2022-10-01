use proconio::{fastout, input};

#[allow(non_snake_case)]
#[allow(clippy::needless_range_loop)]
#[fastout]

fn main() {
    input! {
        N: usize,
        ABCD: [(usize, usize, usize, usize); N]
    }
    const MAX: usize = 1500;

    let mut T = vec![vec![0; MAX + 1]; MAX + 1];

    for &(A, B, C, D) in &ABCD {
        T[A][B] += 1;
        T[C][B] -= 1;
        T[A][D] -= 1;
        T[C][D] += 1;
    }

    for i in 0..=MAX {
        for j in 1..=MAX {
            T[i][j] += T[i][j - 1];
        }
    }
    for j in 0..=MAX {
        for i in 1..=MAX {
            T[i][j] += T[i - 1][j];
        }
    }

    let mut ans = 0;
    for i in 0..=MAX {
        for j in 0..=MAX {
            if T[i][j] > 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
