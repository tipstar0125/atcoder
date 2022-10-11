use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]

fn main() {
    input! {
        Q: usize,
        X: [usize; Q]
    }

    for &x in &X {
        let mut ans = "Yes";
        let mut i = 2;
        while i * i <= x {
            if x % i == 0 {
                ans = "No";
            }
            i += 1;
        }
        println!("{}", ans);
    }
}
