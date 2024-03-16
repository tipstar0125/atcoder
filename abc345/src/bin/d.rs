#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet, VecDeque},
};
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
            N: usize,
            H: usize,
            W: usize,
            AB: [(usize, usize); N]
        }
        let mut board = vec![vec![!0; W]; H];
        let mut unused = HashSet::new();
        for i in 0..N {
            unused.insert(i);
        }
        let ans = dfs(&mut board, &mut unused, 0, 0, H, W, &AB);
        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn dfs(
    board: &mut [Vec<usize>],
    unused: &mut HashSet<usize>,
    mut row: usize,
    mut col: usize,
    H: usize,
    W: usize,
    AB: &[(usize, usize)],
) -> bool {
    while board[row][col] != !0 {
        col += 1;
        if col == W {
            col = 0;
            row += 1;
        }
        if row == H {
            break;
        }
    }
    if row == H {
        return true;
    }

    for &n in unused.clone().iter() {
        if row + AB[n].0 <= H && col + AB[n].1 <= W {
            let mut ok = true;
            'outer: for i in 0..AB[n].0 {
                for j in 0..AB[n].1 {
                    if board[row + i][col + j] != !0 {
                        ok = false;
                        break 'outer;
                    }
                }
            }
            if ok {
                unused.remove(&n);
                for i in 0..AB[n].0 {
                    for j in 0..AB[n].1 {
                        board[row + i][col + j] = n;
                    }
                }
                let ret = dfs(board, unused, row, col, H, W, AB);
                if ret {
                    return true;
                }
                unused.insert(n);
                for i in 0..AB[n].0 {
                    for j in 0..AB[n].1 {
                        board[row + i][col + j] = !0;
                    }
                }
            }
        }
        if AB[n].0 == AB[n].1 {
            continue;
        }
        if row + AB[n].1 <= H && col + AB[n].0 <= W {
            let mut ok = true;
            'outer: for i in 0..AB[n].1 {
                for j in 0..AB[n].0 {
                    if board[row + i][col + j] != !0 {
                        ok = false;
                        break 'outer;
                    }
                }
            }
            if ok {
                unused.remove(&n);
                for i in 0..AB[n].1 {
                    for j in 0..AB[n].0 {
                        board[row + i][col + j] = n;
                    }
                }
                let ret = dfs(board, unused, row, col, H, W, AB);
                if ret {
                    return true;
                }
                unused.insert(n);
                for i in 0..AB[n].1 {
                    for j in 0..AB[n].0 {
                        board[row + i][col + j] = !0;
                    }
                }
            }
        }
    }
    false
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
