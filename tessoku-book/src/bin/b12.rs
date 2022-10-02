use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: f64,
    }

    let mut left = 0_f64;
    let mut right = 100_f64;

    for _ in 0..20 {
        let med = (left + right) / 2.0;
        let value = med.powf(3.0) + med;
        if value < N {
            left = med;
        } else {
            right = med;
        }
    }

    println!("{}", left);
}
