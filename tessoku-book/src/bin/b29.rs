use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize
    }

    let m = 1000000007;
    let mut ans = 1;
    let mut p = a;
    for i in 0..64 {
        if b >> i & 1 == 1 {
            ans = (ans * p) % m;
        }
        p = (p * p) % m;
    }

    println!("{:?}", ans);
}
