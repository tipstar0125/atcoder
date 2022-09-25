use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        A: usize,
        B: usize
    }

    println!("{:?}", A + B);
}
