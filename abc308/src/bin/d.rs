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
            H: usize,
            W: usize,
            S: [Chars; H]
        }

        if S[0][0] != 's' {
            println!("No");
            return;
        }
        let mut visited = vec![vec![false; W]; H];
        let mut ok = false;
        visited[0][0] = true;
        dfs(0, 0, H, W, &S, 's', &mut visited, &mut ok);
        if ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
const DIJ4: [(usize, usize); 4] = [(!0, 0), (0, !0), (1, 0), (0, 1)];

fn dfs(
    x: usize,
    y: usize,
    H: usize,
    W: usize,
    S: &Vec<Vec<char>>,
    c: char,
    visited: &mut Vec<Vec<bool>>,
    ok: &mut bool,
) {
    if x == H - 1 && y == W - 1 {
        *ok = true;
        return;
    }
    let mut mp: BTreeMap<char, char> = BTreeMap::new();
    mp.insert('s', 'n');
    mp.insert('n', 'u');
    mp.insert('u', 'k');
    mp.insert('k', 'e');
    mp.insert('e', 's');

    for &(dx, dy) in &DIJ4 {
        let x1 = x.wrapping_add(dx);
        let y1 = y.wrapping_add(dy);
        if x1 < H && y1 < W && !visited[x1][y1] && S[x1][y1] == mp[&c] {
            visited[x1][y1] = true;
            dfs(x1, y1, H, W, S, mp[&c], visited, ok);
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
