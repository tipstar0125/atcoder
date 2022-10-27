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

    let mut dist = vec![vec![-1; C]; R];
    let mut Q = VecDeque::new();
    for i in 0..R {
        for j in 0..C {
            if c[i][j] == '#' {
                dist[i][j] = 0;
            }
        }
    }
    dist[sy - 1][sx - 1] = 0;
    Q.push_back((sy - 1, sx - 1));

    while !Q.is_empty() {
        let (y, x) = Q.pop_front().unwrap();
        if dist[y + 1][x] == -1 {
            Q.push_back((y + 1, x));
            dist[y + 1][x] = dist[y][x] + 1;
        }
        if dist[y - 1][x] == -1 {
            Q.push_back((y - 1, x));
            dist[y - 1][x] = dist[y][x] + 1;
        }
        if dist[y][x + 1] == -1 {
            Q.push_back((y, x + 1));
            dist[y][x + 1] = dist[y][x] + 1;
        }
        if dist[y][x - 1] == -1 {
            Q.push_back((y, x - 1));
            dist[y][x - 1] = dist[y][x] + 1;
        }
    }
    println!("{}", dist[gy - 1][gx - 1]);
}
