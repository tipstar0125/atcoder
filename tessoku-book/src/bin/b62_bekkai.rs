#![allow(non_snake_case)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_traits::ops::checked;
use proconio::{fastout, input, marker::Chars};
use superslice::Ext;

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
    let mut memo = vec![0; N + 1];
    let mut finish = false;
    dfs(1, &G, &mut visited, &mut memo, 1, &mut finish);
    println!(
        "{}",
        memo[1..=memo.iter().position(|&x| x == N).unwrap()]
            .iter()
            .join(" ")
    );
}

fn dfs(
    pos: usize,
    G: &[Vec<usize>],
    visited: &mut [bool],
    memo: &mut [usize],
    index: usize,
    finish: &mut bool,
) {
    visited[pos] = true;
    if !*finish {
        memo[index] = pos;
    }
    if pos == visited.len() - 1 {
        *finish = true;
    }
    for &next in &G[pos] {
        if !visited[next] {
            dfs(next, G, visited, memo, index + 1, finish);
        }
    }
}
