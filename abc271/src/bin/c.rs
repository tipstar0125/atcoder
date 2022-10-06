use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        mut a: [usize; N]
    }
    a.sort();
    a.dedup();
    while a.len() < N {
        a.push(0);
    }

    let mut target = 1;
    let mut ans = 0;
    let mut continues = true;
    let mut i = 0;

    while continues {
        if a[i] == target {
            ans += 1;
            i += 1;
            if a.len() < i + 1 {
                continues = false;
            }
        } else if a.len() >= i + 2 {
            a.pop();
            a.pop();
            ans += 1;
            if a.len() < i + 1 {
                continues = false;
            }
        } else {
            continues = false;
        }

        target += 1;
    }

    println!("{:?}", ans);
}
