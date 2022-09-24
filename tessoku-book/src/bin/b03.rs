use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let mut ans = false;
    for i in 0..N {
        for j in i+1..N {
            for k in j+1..N {
                if A[i] + A[j] + A[k] == 1000 {
                    ans = true;
                }
            }
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
