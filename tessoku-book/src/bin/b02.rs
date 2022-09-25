use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        A: usize,
        B: usize
    }
    let mut ans = false;

    for i in A..=B {
        if (100 % i) == 0 {
            ans = true;
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
