#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
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
#[macro_export]
macro_rules! abs {
    ($x: expr) => {
        if $x < 0_isize {
            $x * (-1)
        } else {
            $x
        }
    };
}
#[macro_export]
macro_rules! absf {
    ($x: expr) => {
        if $x < 0.0 {
            $x * (-1.0)
        } else {
            $x
        }
    };
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

    fn unite(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return;
        }
        let size_x = -self.parent[root_x];
        let size_y = -self.parent[root_y];
        if size_x >= size_y {
            self.parent[root_x] -= size_y;
            self.parent[root_y] = root_x as isize;
        } else {
            self.parent[root_y] -= size_x;
            self.parent[root_x] = root_y as isize;
        }
        self.size -= 1;
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
            x: isize,
            y: isize,
            A: [usize; N]
        }
        let dx = abs!(x - A[0] as isize) as usize;
        let dy = abs!(y) as usize;
        let mut x_list = vec![];
        let mut y_list = vec![];
        for (i, &a) in A[1..].iter().enumerate() {
            if i % 2 == 0 {
                y_list.push(a);
            } else {
                x_list.push(a);
            }
        }

        let MAX = 10000;
        let mut dp_x = vec![vec![false; MAX + 1]; x_list.len() + 1];
        dp_x[0][x_list.iter().sum::<usize>()] = true;
        for i in 1..=x_list.len() {
            let x = x_list[i - 1];
            for j in 0..=MAX {
                if dp_x[i - 1][j] {
                    dp_x[i][j] = true;
                    dp_x[i][abs!(j as isize - (x as isize) * 2) as usize] = true;
                }
            }
        }

        let mut dp_y = vec![vec![false; MAX + 1]; y_list.len() + 1];
        dp_y[0][y_list.iter().sum::<usize>()] = true;
        for i in 1..=y_list.len() {
            let y = y_list[i - 1];
            for j in 0..=MAX {
                if dp_y[i - 1][j] {
                    dp_y[i][j] = true;
                    dp_y[i][abs!(j as isize - (y as isize) * 2) as usize] = true;
                }
            }
        }

        if dp_x[x_list.len()][dx] && dp_y[y_list.len()][dy] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
