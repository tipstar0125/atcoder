#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

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
#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<isize>,
    size: usize,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![-1; n + 1],
            size: n,
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            return x;
        }
        let root = self.find(self.parent[x] as usize);
        self.parent[x] = root as isize;
        root
    }
    fn unite(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return None;
        }
        let size_x = -self.parent[root_x];
        let size_y = -self.parent[root_y];
        self.size -= 1;
        if size_x >= size_y {
            self.parent[root_x] -= size_y;
            self.parent[root_y] = root_x as isize;
            Some((root_x, root_y))
        } else {
            self.parent[root_y] -= size_x;
            self.parent[root_x] = root_y as isize;
            Some((root_y, root_x))
        }
    }
    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    fn is_root(&mut self, x: usize) -> bool {
        self.find(x) == x
    }
    fn get_union_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        -self.parent[root] as usize
    }
    fn get_size(&self) -> usize {
        self.size
    }
}
#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize,
            A: [usize; N]
        }

        if A.iter().sum::<usize>() % 10 != 0 {
            println!("No");
            return;
        }

        let cut_size = A.iter().sum::<usize>() / 10;
        let mut doubleA = A.clone();
        doubleA.extend(&A);
        let mut check_list = vec![];

        if A.iter().all(|&x| x <= cut_size) {
            check_list.push(doubleA);
        } else {
            let mut start = 0;
            while A[start] <= cut_size {
                start += 1;
            }
            start += 1;

            let mut vec = vec![];
            for i in 0..N {
                let pos = (i + start) % N;
                if A[pos] <= cut_size {
                    vec.push(A[pos]);
                } else {
                    check_list.push(vec);
                    vec = vec![];
                }
            }
            check_list.push(vec);
        }

        let mut ans = false;
        for list in check_list {
            if list.is_empty() {
                continue;
            }
            let mut l = 0;
            let mut r = 0;
            let mut sum = list[0];
            let mut ok = false;
            loop {
                if sum == cut_size {
                    ok = true;
                    break;
                } else if sum < cut_size {
                    r += 1;
                    if r >= list.len() {
                        break;
                    }
                    sum += list[r];
                } else {
                    if l >= list.len() {
                        break;
                    }
                    sum -= list[l];
                    l += 1;
                }
            }
            ans |= ok;
        }
        println!("{}", if ans { "Yes" } else { "No" });
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
