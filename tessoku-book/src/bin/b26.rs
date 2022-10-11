use proconio::{fastout, input};

#[allow(non_snake_case)]
#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        N: usize
    }

    let mut is_prime_list = vec![true; N + 1];

    let mut i = 2;
    while i * i <= N {
        if is_prime_list[i] {
            let mut j = i * 2;
            while j <= N {
                is_prime_list[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    for i in 2..=N {
        if is_prime_list[i] {
            println!("{:?}", i);
        }
    }
}
