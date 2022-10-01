use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
        ABCD: [(usize, usize, usize, usize); N]
    }

    let mut X = vec![vec![0; W + 2]; H + 2];

    for &(A, B, C, D) in &ABCD {
        X[A][B] += 1;
        X[A][D + 1] -= 1;
        X[C + 1][B] -= 1;
        X[C + 1][D + 1] += 1;
    }

    let mut Z = vec![vec![0; W + 1]; H + 1];
    for i in 1..=H {
        for j in 1..=W {
            Z[i][j] = Z[i][j - 1] + X[i][j];
        }
    }
    for j in 1..=W {
        for i in 1..=H {
            Z[i][j] += Z[i - 1][j];
        }
    }

    for row in &Z[1..] {
        let ans = row[1..]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", ans);
    }
}
