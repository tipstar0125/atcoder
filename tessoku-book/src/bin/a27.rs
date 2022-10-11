use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        mut A: usize,
        mut B: usize,
    }

    while A > 0 && B > 0 {
        if A >= B {
            A %= B;
        } else {
            B %= A;
        };
    }

    if A == 0 {
        println!("{:?}", B);
    } else {
        println!("{:?}", A);
    }
}
