#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};
use superslice::Ext;

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

const DIJ4: [(usize, usize); 4] = [(0, 1), (1, 0), (!0, 0), (0, !0)];

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize,
        }
        let mut ans = vec![vec![0; N]; N];
        let mut cnt = 1;
        let mut x = 0;
        let mut y = 0;
        let mut dir = 0;
        ans[x][y] = cnt;
        cnt += 1;
        while cnt <= N * N {
            loop {
                let nx = x.wrapping_add(DIJ4[dir % 4].0);
                let ny = y.wrapping_add(DIJ4[dir % 4].1);
                if nx >= N || ny >= N || ans[nx][ny] != 0 {
                    break;
                }
                ans[nx][ny] = cnt;
                cnt += 1;
                x = nx;
                y = ny;
            }
            dir += 1;
        }
        for i in 0..N {
            for j in 0..N {
                if ans[i][j] != N * N {
                    print!("{}", ans[i][j]);
                } else {
                    print!("T");
                }
                if j != N - 1 {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
