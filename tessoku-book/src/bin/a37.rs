use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        B: usize,
        A: [usize; N],
        C: [usize; M]
    }

    let mut ans = A.iter().sum::<usize>() * M;
    ans += C.iter().sum::<usize>() * N;
    ans += B * N * M;
    println!("{:?}", ans);
}
