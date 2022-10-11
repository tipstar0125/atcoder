use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        _: usize,
        K: usize,
        S: Chars
    }

    let mut num_on = 0;
    for s in S {
        if s == '1' {
            num_on += 1;
        }
    }

    if K % 2 == num_on % 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
