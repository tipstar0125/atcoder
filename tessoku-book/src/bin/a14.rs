use proconio::{fastout, input};
use superslice::Ext;

#[allow(non_snake_case)]
#[allow(clippy::needless_range_loop)]
#[fastout]

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
        B: [usize; N],
        C: [usize; N],
        D: [usize; N]
    }

    let mut P: Vec<_> = vec![];
    let mut Q: Vec<_> = vec![];

    for i in 0..N {
        for j in 0..N {
            P.push(A[i] + B[j]);
            Q.push(C[i] + D[j]);
        }
    }

    P.sort();
    Q.sort();

    let mut ans = "No";
    for p in &P {
        let target = K - p;
        let index = Q.lower_bound(&target);
        if index < N * N && p + Q[index] == K {
            ans = "Yes";
        }
    }
    println!("{}", ans);
}
