use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: Chars
    }
    
    let mut ans = 0;
    for (i, &c) in N.iter().rev().enumerate() {
        if c == '1' {
           ans += 1 << i;
        }
    }
    println!("{:?}", ans);
}
