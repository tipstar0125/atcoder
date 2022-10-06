use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N]
    }

    let mut left = 0;
    let mut right = 1_000_000_000;

    while right - left > 1 {
        let med = (left + right) / 2;
        let mut sum = 0;
        for a in &A {
            sum += med / a;
        }
        if sum >= K {
            right = med;
        } else {
            left = med;
        }
    }

    println!("{}", right);
}
