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
            Q: usize,
            mut A: [usize; N],
        }

        let mut cnt = vec![0; N + 1];
        for &a in &A {
            if a > N {
                continue;
            }
            cnt[a] += 1;
        }
        let mut heap = BinaryHeap::new();
        for i in 0..=N {
            if cnt[i] == 0 {
                heap.push(Reverse(i));
            }
        }
        for _ in 0..Q {
            input! {
                i: Usize1,
                x: usize
            }
            if A[i] <= N {
                cnt[A[i]] -= 1;
            }
            if x <= N {
                cnt[x] += 1;
            }

            if A[i] <= N && cnt[A[i]] == 0 {
                heap.push(Reverse(A[i]));
            }
            while let Some(Reverse(y)) = heap.peek() {
                if cnt[*y] > 0 {
                    heap.pop();
                } else {
                    break;
                }
            }
            A[i] = x;
            let Reverse(ans) = *heap.peek().unwrap();
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
