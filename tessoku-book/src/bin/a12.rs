use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N]
    }

    let mut left = 1;
    let mut right = 1_000_000_000;

    while left < right {
        let med = (left + right) / 2;
        let mut sum = 0;
        for a in &A {
            sum += med / a;
        }
        if sum >= K {
            right = med;
        } else {
            left = med + 1;
        }
    }

    println!("{}", left);
}
