use std::collections::{HashMap, HashSet};

use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(usize, usize); M]
    }

    let mut map = HashMap::new();

    for &(A, B) in &AB {
        map.entry(A).or_insert(vec![]).push(B);
        map.entry(B).or_insert(vec![]).push(A);
    }

    for i in 1..=N {
        let vec = map.get(&i).cloned().unwrap_or_default();
        let ans: HashSet<_> = vec.iter().cloned().collect();
        println!("{:?}: {:?}", i, ans);
    }
}
