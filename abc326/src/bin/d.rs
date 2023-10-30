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
            N: usize,
            R: Chars,
            C: Chars
        }

        let mut set = BTreeSet::new();
        set.insert('A');
        set.insert('B');
        set.insert('C');

        let mut vv = vec![vec![]; N];

        for i in 0..N {
            for c in (0..N).combinations(3) {
                let mut row = vec!['.'; N];
                let mut s = set.clone();
                row[c[0]] = R[i];
                s.remove(&R[i]);
                let first = s.pop_first().unwrap();
                let last = s.pop_last().unwrap();
                row[c[1]] = first;
                row[c[2]] = last;
                vv[i].push(row.clone());
                row[c[2]] = first;
                row[c[1]] = last;
                vv[i].push(row.clone());
            }
        }

        let mut Q: Vec<Vec<Vec<char>>> = vec![];
        Q.push(vec![]);
        while let Some(v) = Q.pop() {
            let c = v.len();
            if c == N {
                let mut ok = true;
                for j in 0..N {
                    let mut cnt = vec![0; 3];
                    let mut check = true;
                    for i in 0..N {
                        if v[i][j] == '.' {
                            continue;
                        }
                        let n = v[i][j] as u8 - b'A';
                        if check {
                            if C[j] != v[i][j] {
                                ok = false;
                            }
                            check = false;
                        }
                        cnt[n as usize] += 1;
                    }
                    if cnt[0] != 1 || cnt[1] != 1 || cnt[2] != 1 {
                        ok = false;
                    }
                }
                if ok {
                    println!("Yes");
                    for row in &v {
                        println!("{}", row.iter().join(""));
                    }
                    return;
                }
                continue;
            }
            for row in &vv[c] {
                let mut nv = v.clone();
                nv.push(row.clone());
                Q.push(nv);
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
