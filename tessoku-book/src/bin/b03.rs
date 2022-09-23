use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: u32,
        A: [u32; N]
    }

    let mut exists = false;
    for i in 0..A.len() {
        for j in i+1..A.len() {
            for k in j+1..A.len() {
                if A[i] + A[j] + A[k] == 1000 {
                    exists = true;
                }
            }
        }
    }

    if exists {
        println!("Yes");
    } else {
        println!("No");
    }
}
