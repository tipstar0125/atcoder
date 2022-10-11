use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize
    }

    let mut ans = 1;
    let c = b / 5;
    let d = b % 5;
    let a_pow5 = a.pow(5);

    for _ in 1..=c {
        ans *= a_pow5;
        ans %= 1000000007;
    }
    for _ in 1..=d {
        ans *= a;
        ans %= 1000000007;
    }
    println!("{:?}", ans);
}
