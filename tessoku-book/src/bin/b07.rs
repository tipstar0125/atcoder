use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
        N: usize,
        LR: [(usize, usize); N]
    }

    let mut B = vec![0; T + 1];
    for &(L, R) in &LR {
        B[L] += 1;
        B[R] -= 1;
    }

    let mut ans = 0;
    for b in &B[..T] {
        ans += b;
        println!("{:?}", ans);
    }
}
