use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: isize,
        K: isize
    }

    let mut ans = 0;
    for i in 1..=N {
        for j in 1..=N {
            let x = K - i - j;
            if x >= 1 && x <= N {
                ans += 1;
            }
        }
    }

    println!("{:?}", ans);
}
