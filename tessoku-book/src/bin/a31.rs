use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N:usize
    }

    let num_3 = N / 3;
    let num_5 = N / 5;
    let num_15 = N / 15;
    println!("{}", num_3 + num_5 - num_15);
}
