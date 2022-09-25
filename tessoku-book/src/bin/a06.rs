use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
     N: usize,
     Q: usize,
     A: [usize; N],
     LR: [(usize, usize); Q]
    }

    let mut S = vec![0];

    for i in 0..N {
        S.push(A[i] + S[i])
    }

    for (L, R) in &LR {
        println!("{:?}", S[*R] - S[*L - 1]);
    }
}
