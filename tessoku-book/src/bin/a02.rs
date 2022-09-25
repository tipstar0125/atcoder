use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        X: usize,
        A: [usize; N]
    }
    let mut ans = false;

    for a in &A {
        if *a == X {
            ans = true;
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
