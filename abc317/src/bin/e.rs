#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use amplify::confinement::Collection;
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
            H: usize,
            W: usize,
            A: [Chars; H]
        }

        let mut visited = vec![vec![false; W]; H];
        for i in 0..H {
            let mut b = false;
            for j in 0..W {
                if A[i][j] == '>' {
                    visited[i][j] = true;
                    b = true;
                } else if A[i][j] != '.' && A[i][j] != 'S' && A[i][j] != 'G' {
                    b = false;
                }
                if b {
                    visited[i][j] = true;
                }
            }
        }
        for i in 0..H {
            let mut b = false;
            for j in (0..W).rev() {
                if A[i][j] == '<' {
                    visited[i][j] = true;
                    b = true;
                } else if A[i][j] != '.' && A[i][j] != 'S' && A[i][j] != 'G' {
                    b = false;
                }
                if b {
                    visited[i][j] = true;
                }
            }
        }
        for j in 0..W {
            let mut b = false;
            for i in 0..H {
                if A[i][j] == 'v' {
                    visited[i][j] = true;
                    b = true;
                } else if A[i][j] != '.' && A[i][j] != 'S' && A[i][j] != 'G' {
                    b = false;
                }
                if b {
                    visited[i][j] = true;
                }
            }
        }
        for j in 0..W {
            let mut b = false;
            for i in (0..H).rev() {
                if A[i][j] == '^' {
                    visited[i][j] = true;
                    b = true;
                } else if A[i][j] != '.' && A[i][j] != 'S' && A[i][j] != 'G' {
                    b = false;
                }
                if b {
                    visited[i][j] = true;
                }
            }
        }
        let mut start = (0, 0);
        let mut goal = (0, 0);
        for i in 0..H {
            for j in 0..W {
                if A[i][j] == '#' {
                    visited[i][j] = true;
                }
                if A[i][j] == 'S' {
                    start = (i, j);
                }
                if A[i][j] == 'G' {
                    goal = (i, j);
                }
            }
        }

        let INF = 1_usize << 60;
        let mut dist = vec![vec![INF; W]; H];
        let mut Q = VecDeque::new();
        dist[start.0][start.1] = 0;
        Q.push_back(start);
        while let Some((r, c)) = Q.pop_front() {
            for &(dr, dc) in &DIJ4 {
                let row = r.wrapping_add(dr);
                let col = c.wrapping_add(dc);
                if row >= H || col >= W {
                    continue;
                }
                if !visited[row][col] {
                    dist[row][col] = dist[r][c] + 1;
                    visited[row][col] = true;
                    Q.push_back((row, col));
                }
            }
        }
        let ans = dist[goal.0][goal.1];
        if ans >= INF {
            println!("-1");
        } else {
            println!("{}", ans);
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
