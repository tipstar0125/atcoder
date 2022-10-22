#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        N: usize,
        P: usize,
        Q: usize,
        R: usize,
        A: [usize; N]
    }

    let mut S = vec![0; N + 1];
    for i in 1..=N {
        S[i] = S[i - 1] + A[i - 1];
    }

    let mut exists = false;

    for i in 0..=N - 3 {
        let j = S.lower_bound(&(P + S[i]));
        if j <= N && S[j] - S[i] == P {
            let k = S.lower_bound(&(Q + S[j]));
            if k <= N && S[k] - S[j] == Q {
                let l = S.lower_bound(&(R + S[k]));
                if l <= N && S[l] - S[k] == R {
                    exists = true;
                }
            }
        }
    }

    if exists {
        println!("Yes");
    } else {
        println!("No");
    }
}
