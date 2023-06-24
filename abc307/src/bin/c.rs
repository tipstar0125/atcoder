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

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            HA: usize,
            WA: usize,
            A: [Chars; HA],
            HB: usize,
            WB: usize,
            B: [Chars; HB],
            HX: usize,
            WX: usize,
            X: [Chars; HX],
        }

        let mut C = vec![vec!['.'; 50]; 50];
        for i in 0..HA {
            for j in 0..WA {
                C[i + 20][j + 20] = A[i][j];
            }
        }

        for i in 0..50 - HB {
            for j in 0..50 - WB {
                let mut CC = C.clone();
                for k in 0..HB {
                    for l in 0..WB {
                        if B[k][l] == '#' {
                            CC[i + k][j + l] = '#';
                        }
                    }
                }

                let mut c = 0;
                for k in 0..50 {
                    for l in 0..50 {
                        if CC[k][l] == '#' {
                            c += 1;
                        }
                    }
                }
                for k in 0..50 - HX {
                    for l in 0..50 - WX {
                        let mut D = vec![vec!['.'; WX]; HX];
                        for m in 0..HX {
                            for n in 0..WX {
                                D[m][n] = CC[k + m][l + n];
                            }
                        }
                        let mut cnt = c;
                        for m in 0..HX {
                            for n in 0..WX {
                                if D[m][n] == '#' {
                                    cnt -= 1;
                                }
                            }
                        }
                        if D == X && cnt == 0 {
                            println!("Yes");
                            return;
                        }
                    }
                }
            }
        }
        println!("No");
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
