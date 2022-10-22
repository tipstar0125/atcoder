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

    let mut visited = vec![false; N + 1];
    dfs(1, &G, &mut visited);

    if visited[1..].iter().all(|&x| x) {
        println!("The graph is connected.");
    } else {
        println!("The graph is not connected.");
    }
}

fn dfs(pos: usize, G: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[pos] = true;
    for &next in &G[pos] {
        if !visited[next] {
            dfs(next, G, visited);
        }
    }
}
