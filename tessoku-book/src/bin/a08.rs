use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        matrix : [[usize; W]; H],
        Q: usize,
        ABCD : [(usize, usize, usize, usize); Q]
    }

    let mut Z = vec![vec![0; W + 1]; H + 1];

    for i in 1..=H {
        for j in 1..=W {
            Z[i][j] = Z[i][j - 1] + matrix[i - 1][j - 1];
        }
    }

    for j in 1..=W {
        for i in 1..=H {
            Z[i][j] += Z[i - 1][j];
        }
    }

    for &(A, B, C, D) in &ABCD {
        let ans = Z[C][D] + Z[A - 1][B - 1] - Z[A - 1][D] - Z[C][B - 1];
        println!("{:?}", ans);
    }
}
