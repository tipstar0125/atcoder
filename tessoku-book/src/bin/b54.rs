use std::collections::HashMap;

use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let mut map = HashMap::new();

    let mut ans = 0;
    for a in &A {
        *map.entry(a).or_insert(0) += 1_usize;
        ans += map[a] - 1;
    }

    println!("{:?}", ans);
}
