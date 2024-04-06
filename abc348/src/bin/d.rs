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
            H: usize,
            W: usize,
            A: [Chars; H],
            N: usize,
            RCE: [(Usize1, Usize1, isize); N]
        }

        let mut E = vec![vec![0; W]; H];
        for &(r, c, e) in RCE.iter() {
            E[r][c] = e;
        }
        let mut st_r = 0;
        let mut st_c = 0;
        for i in 0..H {
            for j in 0..W {
                if A[i][j] == 'S' {
                    st_r = i;
                    st_c = j;
                }
            }
        }
        if E[st_r][st_c] == 0 {
            println!("No");
            return;
        }
        let e = E[st_r][st_c];
        let mut energy = vec![vec![-1; W]; H];
        energy[st_r][st_c] = e;
        let mut Q = BinaryHeap::new();
        Q.push((e, st_r, st_c));
        while let Some((e, pi, pj)) = Q.pop() {
            if A[pi][pj] == 'T' {
                println!("Yes");
                return;
            }
            if energy[pi][pj] != e {
                continue;
            }
            if e == 0 {
                continue;
            }
            for &(di, dj) in &DIJ4 {
                let ni = pi.wrapping_add(di);
                let nj = pj.wrapping_add(dj);
                if ni >= H || nj >= W {
                    continue;
                }
                if A[ni][nj] == '#' {
                    continue;
                }
                if max!(energy[pi][pj] - 1, E[ni][nj]) > energy[ni][nj] {
                    energy[ni][nj] = max!(energy[pi][pj] - 1, E[ni][nj]);
                    Q.push((energy[ni][nj], ni, nj))
                }
            }
        }
        println!("No");
    }
}

#[macro_export]
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $( $y: expr ),+) => {
        std::cmp::max($x, max!($( $y ),+))
    }
}
#[macro_export]
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $( $y: expr ),+) => {
        std::cmp::min($x, min!($( $y ),+))
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
