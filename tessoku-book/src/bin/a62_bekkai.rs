#![allow(non_snake_case)]
#![allow(unused_imports)]
use num_traits::ops::checked;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(usize, usize); M]
    }

    let mut G = vec![vec![]; N + 1];
    for &(A, B) in &AB {
        G[A].push(B);
        G[B].push(A);
    }

    let mut stack = vec![1];
    let mut visited = vec![false; N + 1];

    while !stack.is_empty() {
        let x = stack.pop().unwrap();
        let vec = G.get(x).unwrap();
        visited[x] = true;

        for &v in vec {
            if !visited[v] {
                stack.push(v);
            }
        }
    }

    if visited[1..].iter().all(|&x| x) {
        println!("The graph is connected.");
    } else {
        println!("The graph is not connected.");
    }
}
