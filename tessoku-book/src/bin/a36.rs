use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize
    }

    if N * 2 - 2 <= K && K % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
