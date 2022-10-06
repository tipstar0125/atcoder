use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        a: [usize; N]
    }

    let mut ok = 0;
    let mut ng = N + 1;

    let mut b = vec![false; N + 1];
    for &x in &a {
        if x < N + 1 {
            b[x] = true;
        }
    }

    while ng - ok > 1 {
        let med = (ok + ng) / 2;
        let mut sum = 0;
        for i in 1..=med {
            if b[i] {
                sum += 1;
            }
        }

        if sum + (N - sum) / 2 >= med {
            ok = med;
        } else {
            ng = med;
        }
    }
    println!("{:?}", ok);
}
