#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
use std::collections::VecDeque;

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

pub mod macros {
    #[macro_export]
    macro_rules !max {
        ($x: expr) => ($x);
        ($x: expr, $($y: expr), +) => {
            std::cmp::max($x, max!($($y), +))
        }
    }
    #[macro_export]
    macro_rules !min {
        ($x: expr) => ($x);
        ($x: expr, $($y: expr), +) => {
            std::cmp::min($x, min!($($y), +))
        }
    }
}

#[fastout]
fn main() {
    input! {
        R: usize,
        C: usize,
        sy: usize,
        sx: usize,
        gy: usize,
        gx: usize,
        c: [Chars; R]
    }

    let start = (sy - 1) * C + (sx - 1);
    let goal = (gy - 1) * C + (gx - 1);

    let mut G = vec![vec![]; R * C + 1];
    for i in 0..R {
        for j in 0..C - 1 {
            let c1 = c[i][j];
            let c2 = c[i][j + 1];
            let idx1 = i * C + j;
            let idx2 = i * C + j + 1;
            if c1 == '.' && c2 == '.' {
                G[idx1].push(idx2);
                G[idx2].push(idx1);
            }
        }
    }
    for j in 0..C {
        for i in 0..R - 1 {
            let c1 = c[i][j];
            let c2 = c[i + 1][j];
            let idx1 = i * C + j;
            let idx2 = (i + 1) * C + j;
            if c1 == '.' && c2 == '.' {
                G[idx1].push(idx2);
                G[idx2].push(idx1);
            }
        }
    }

    let mut Q = VecDeque::new();
    let mut dist = vec![-1; R * C + 1];
    Q.push_back(start);
    dist[start] = 0;

    while !Q.is_empty() {
        let pos = Q.pop_front().unwrap();
        for &to in &G[pos] {
            if dist[to] == -1 {
                Q.push_back(to);
                dist[to] = dist[pos] + 1;
            }
        }
    }
    println!("{}", dist[goal]);
}
