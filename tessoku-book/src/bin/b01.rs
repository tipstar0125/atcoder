use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize
    }

    println!("{:?}", A + B);
}
