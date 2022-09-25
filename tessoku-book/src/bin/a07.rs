use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        D: usize,
        N: usize,
        LR: [(usize,usize); N]
    }

    let mut B = vec![0; D + 1];

    for &(L, R) in &LR {
        B[L - 1] += 1;
        B[R] -= 1;
    }

    let mut ans = 0;
    for b in &B[..D] {
        ans += b;
        println!("{:?}", ans);
    }
}
