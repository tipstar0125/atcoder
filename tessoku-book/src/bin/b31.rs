use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N:usize
    }

    let num_3 = N / 3;
    let num_5 = N / 5;
    let num_7 = N / 7;
    let num_15 = N / 15;
    let num_21 = N / 21;
    let num_35 = N / 35;
    let num_105 = N / 105;

    println!(
        "{}",
        num_3 + num_5 + num_7 - num_15 - num_21 - num_35 + num_105
    );
}
