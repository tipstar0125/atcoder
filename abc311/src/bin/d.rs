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

const DIJ4: [(usize, usize); 4] = [(!0, 0), (0, !0), (1, 0), (0, 1)];

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize,
            M: usize,
            S: [Chars; N]
        }

        let mut stop = vec![vec![false; M]; N];
        let mut visited = vec![vec![false; M]; N];
        let mut Q = VecDeque::new();
        Q.push_back((1_usize, 1_usize));
        stop[1][1] = true;
        visited[1][1] = true;

        while let Some((r, c)) = Q.pop_front() {
            for &(dr, dc) in &DIJ4 {
                let mut nr = r;
                let mut nc = c;
                while S[nr.wrapping_add(dr)][nc.wrapping_add(dc)] == '.' {
                    nr = nr.wrapping_add(dr);
                    nc = nc.wrapping_add(dc);
                    visited[nr][nc] = true;
                }
                if !stop[nr][nc] {
                    stop[nr][nc] = true;
                    Q.push_back((nr, nc));
                }
            }
        }
        let mut ans = 0;
        for i in 0..N {
            for j in 0..M {
                if visited[i][j] {
                    ans += 1;
                }
            }
        }
        println!("{}", ans);
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
