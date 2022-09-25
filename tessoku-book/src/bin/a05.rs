use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: isize,
        K: isize
    }

    let mut ans = 0;
    for i in 1..=N {
        for j in 1..=N {
            let x = K - i - j;
            if 1 <= x && x <= N {
                ans += 1;
            }
        }
    }

    println!("{:?}", ans);
}
