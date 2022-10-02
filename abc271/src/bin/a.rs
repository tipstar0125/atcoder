use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }
    let num_str = "0123456789ABCDEF";
    let ten = N / 16 % 16;
    let one = N % 16;
    println!("{}{}", num_str.chars().nth(ten).unwrap(), num_str.chars().nth(one).unwrap());
}