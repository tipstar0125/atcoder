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
            T: usize
        }

        'outer: for _ in 0..T {
            input! {
                N: usize,
                X: Chars,
                Y: Chars
            }
            let mut XX = vec![];
            let mut YY = vec![];
            for i in 0..N {
                if X[i] != 'C' && Y[i] == 'C' {
                    println!("No");
                    continue 'outer;
                } else if Y[i] == 'C' {
                    let mut cnt_x = vec![0; 3];
                    for &x in &XX {
                        let n = (x as u8 - b'A') as usize;
                        cnt_x[n] += 1;
                    }
                    let mut cnt_y = vec![0; 3];
                    for &y in &YY {
                        let n = (y as u8 - b'A') as usize;
                        cnt_y[n] += 1;
                    }

                    if cnt_x[0] + cnt_x[2] < cnt_y[0] {
                        println!("No");
                        continue 'outer;
                    }
                    let L = XX.len();
                    let mut idx_A = vec![];
                    let mut idx_B = vec![];
                    for j in 0..L {
                        if XX[j] == 'A' {
                            idx_A.push(j);
                        } else if XX[j] == 'C' && cnt_x[0] < cnt_y[0] {
                            idx_A.push(j);
                            cnt_x[0] += 1;
                        }
                        if YY[j] == 'A' {
                            idx_B.push(j);
                        }
                    }
                    if idx_A.len() != idx_B.len() {
                        println!("No");
                        continue 'outer;
                    }

                    for j in 0..idx_A.len() {
                        if idx_A[j] > idx_B[j] {
                            println!("No");
                            continue 'outer;
                        }
                    }

                    XX.clear();
                    YY.clear();
                    continue;
                }
                XX.push(X[i]);
                YY.push(Y[i]);
            }

            let mut cnt_x = vec![0; 3];
            for &x in &XX {
                let n = (x as u8 - b'A') as usize;
                cnt_x[n] += 1;
            }
            let mut cnt_y = vec![0; 3];
            for &y in &YY {
                let n = (y as u8 - b'A') as usize;
                cnt_y[n] += 1;
            }

            if cnt_x[0] + cnt_x[2] < cnt_y[0] {
                println!("No");
                continue 'outer;
            }
            let L = XX.len();
            let mut idx_A = vec![];
            let mut idx_B = vec![];
            for j in 0..L {
                if XX[j] == 'A' {
                    idx_A.push(j);
                } else if XX[j] == 'C' && cnt_x[0] < cnt_y[0] {
                    idx_A.push(j);
                    cnt_x[0] += 1;
                }
                if YY[j] == 'A' {
                    idx_B.push(j);
                }
            }
            if idx_A.len() != idx_B.len() {
                println!("No");
                continue 'outer;
            }

            for j in 0..idx_A.len() {
                if idx_A[j] > idx_B[j] {
                    println!("No");
                    continue 'outer;
                }
            }

            println!("Yes");
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
