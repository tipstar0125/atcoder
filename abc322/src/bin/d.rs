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
            mut P1: [Chars; 4],
            mut P2: [Chars; 4],
            mut P3: [Chars; 4],
        }

        for _ in 0..4 {
            P1 = rotate(&P1);
            let (rmin1, cmin1, rmax1, cmax1) = size(&P1);
            P1 = shift_row(&P1, rmin1 as isize * (-1));
            P1 = shift_col(&P1, cmin1 as isize * (-1));
            for _ in 0..4 {
                P2 = rotate(&P2);
                let (rmin2, cmin2, rmax2, cmax2) = size(&P2);
                P2 = shift_row(&P2, rmin2 as isize * (-1));
                P2 = shift_col(&P2, cmin2 as isize * (-1));
                for _ in 0..4 {
                    P3 = rotate(&P3);
                    let (rmin3, cmin3, rmax3, cmax3) = size(&P3);
                    P3 = shift_row(&P3, rmin3 as isize * (-1));
                    P3 = shift_col(&P3, cmin3 as isize * (-1));

                    for i1 in 0..=4 - (rmax1 - rmin1 + 1) {
                        P1 = shift_row(&P1, i1 as isize);
                        for j1 in 0..=4 - (cmax1 - cmin1 + 1) {
                            P1 = shift_col(&P1, j1 as isize);

                            for i2 in 0..=4 - (rmax2 - rmin2 + 1) {
                                P2 = shift_row(&P2, i2 as isize);
                                for j2 in 0..=4 - (cmax2 - cmin2 + 1) {
                                    P2 = shift_col(&P2, j2 as isize);

                                    for i3 in 0..=4 - (rmax3 - rmin3 + 1) {
                                        P3 = shift_row(&P3, i3 as isize);
                                        for j3 in 0..=4 - (cmax3 - cmin3 + 1) {
                                            P3 = shift_col(&P3, j3 as isize);

                                            let mut ok = true;
                                            for i in 0..4 {
                                                for j in 0..4 {
                                                    let mut cnt = 0;
                                                    if P1[i][j] == '#' {
                                                        cnt += 1;
                                                    }
                                                    if P2[i][j] == '#' {
                                                        cnt += 1;
                                                    }
                                                    if P3[i][j] == '#' {
                                                        cnt += 1;
                                                    }
                                                    if cnt != 1 {
                                                        ok = false;
                                                    }
                                                }
                                            }
                                            if ok {
                                                println!("Yes");
                                                return;
                                            }

                                            P3 = shift_col(&P3, j3 as isize * (-1));
                                        }
                                        P3 = shift_row(&P3, i3 as isize * (-1));
                                    }

                                    P2 = shift_col(&P2, j2 as isize * (-1));
                                }
                                P2 = shift_row(&P2, i2 as isize * (-1));
                            }

                            P1 = shift_col(&P1, j1 as isize * (-1));
                        }
                        P1 = shift_row(&P1, i1 as isize * (-1));
                    }
                }
            }
        }
        println!("No");
    }
}

fn shift_row(P: &Vec<Vec<char>>, n: isize) -> Vec<Vec<char>> {
    let mut tmp = vec![vec!['.'; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            let pos = i as isize + n;
            if pos < 0 || pos >= 4 {
                continue;
            }
            tmp[pos as usize][j] = P[i][j];
        }
    }
    tmp
}

fn shift_col(P: &Vec<Vec<char>>, n: isize) -> Vec<Vec<char>> {
    let mut tmp = vec![vec!['.'; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            let pos = j as isize + n;
            if pos < 0 || pos >= 4 {
                continue;
            }
            tmp[i][pos as usize] = P[i][j];
        }
    }
    tmp
}

fn size(P: &Vec<Vec<char>>) -> (usize, usize, usize, usize) {
    let mut rows = vec![];
    let mut cols = vec![];
    for i in 0..4 {
        for j in 0..4 {
            if P[i][j] == '#' {
                rows.push(i);
                cols.push(j);
            }
        }
    }
    let rmax = *rows.iter().max().unwrap();
    let rmin = *rows.iter().min().unwrap();
    let cmax = *cols.iter().max().unwrap();
    let cmin = *cols.iter().min().unwrap();
    (rmin, cmin, rmax, cmax)
}

fn rotate<T: Clone + Copy + Default>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let N = v.len();
    let mut ret = vec![vec![T::default(); N]; N];
    for i in 0..N {
        for j in 0..N {
            ret[N - j - 1][i] = v[i][j];
        }
    }
    ret
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
