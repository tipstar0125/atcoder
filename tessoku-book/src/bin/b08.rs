use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        XY: [(usize, usize); N],
        Q: usize,
        abcd: [(usize, usize, usize, usize); Q]
    }
    const MAX: usize = 1500;

    let mut S = vec![vec![0; MAX + 1]; MAX + 1];

    for &(X, Y) in &XY {
        S[X][Y] += 1;
    }

    let mut T = vec![vec![0; MAX + 1]; MAX + 1];

    for i in 1..=MAX {
        for j in 1..=MAX {
            T[i][j] = T[i][j - 1] + S[i][j];
        }
    }
    for j in 1..=MAX {
        for i in 1..=MAX {
            T[i][j] += T[i - 1][j];
        }
    }

    for &(a, b, c, d) in &abcd {
        let ans = T[c][d] + T[a - 1][b - 1] - T[a - 1][d] - T[c][b - 1];
        println!("{:?}", ans);
    }
}
