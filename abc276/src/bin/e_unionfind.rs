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
            H: usize,
            W: usize,
            C: [Chars; H]
        }

        let mut G = vec![vec![]; H * W];
        let mut start_adjacency = vec![];

        for i in 0..H {
            for j in 0..W - 1 {
                if C[i][j] == '.' && C[i][j + 1] == '.' {
                    let index1 = i * W + j;
                    let index2 = i * W + j + 1;
                    G[index1].push(index2);
                    G[index2].push(index1);
                }
                if C[i][j] == '.' && C[i][j + 1] == 'S' {
                    start_adjacency.push(i * W + j);
                }
                if C[i][j] == 'S' && C[i][j + 1] == '.' {
                    start_adjacency.push(i * W + j + 1);
                }
            }
        }
        for j in 0..W {
            for i in 0..H - 1 {
                if C[i][j] == '.' && C[i + 1][j] == '.' {
                    let index1 = i * W + j;
                    let index2 = (i + 1) * W + j;
                    G[index1].push(index2);
                    G[index2].push(index1);
                }
                if C[i][j] == '.' && C[i + 1][j] == 'S' {
                    start_adjacency.push(i * W + j);
                }
                if C[i][j] == 'S' && C[i + 1][j] == '.' {
                    start_adjacency.push((i + 1) * W + j);
                }
            }
        }
        start_adjacency.dedup();
        let mut uf = UnionFind::new(H * W);
        for i in 0..H * W {
            for &x in &G[i] {
                uf.unite(i, x);
            }
        }

        if start_adjacency.len() < 2 {
            println!("No");
            return;
        }

        let mut is_ok = false;
        for i in 0..start_adjacency.len() {
            for j in i + 1..start_adjacency.len() {
                if uf.is_same(start_adjacency[i], start_adjacency[j]) {
                    is_ok = true;
                }
            }
        }
        
        if is_ok {
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
