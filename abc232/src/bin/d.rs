#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use std::collections::{BTreeMap, VecDeque};

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
            H: usize,
            W: usize,
            C: [Chars; H]
        }
        let mut G = vec![vec![]; H * W];
        for i in 0..H {
            for j in 1..W {
                if C[i][j - 1] == '.' && C[i][j] == '.' {
                    let pos0 = i * W + j - 1;
                    let pos1 = i * W + j;
                    G[pos0].push(pos1);
                }
            }
        }
        for j in 0..W {
            for i in 1..H {
                if C[i - 1][j] == '.' && C[i][j] == '.' {
                    let pos0 = (i - 1) * W + j;
                    let pos1 = i * W + j;
                    G[pos0].push(pos1);
                }
            }
        }

        let mut Q = VecDeque::new();
        let mut dist = vec![0; H * W];
        Q.push_back(0);
        dist[0] = 1;
        while !Q.is_empty() {
            let pos = Q.pop_front().unwrap();
            for &nex in &G[pos] {
                if dist[nex] == 0 {
                    dist[nex] = dist[pos] + 1;
                    Q.push_back(nex);
                }
            }
        }
        let ans = dist.iter().max().unwrap();
        println!("{}", ans);
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

fn eratosthenes(n: usize) -> Vec<bool> {
    let mut is_prime_list = vec![true; n + 1];
    is_prime_list[0] = false;
    is_prime_list[1] = false;
    let mut i = 2;
    while i * i <= n {
        if is_prime_list[i] {
            let mut j = i * i;
            while j <= n {
                is_prime_list[j] = false;
                j += i;
            }
        }
        i += 1
    }
    is_prime_list
}

fn mod_pow(a: usize, b: usize, m: usize) -> usize {
    let mut p = a;
    let mut ret = 1;
    let mut n = b;
    while n > 0 {
        if n & 1 == 1 {
            ret = ret * p % m;
        }
        p = p * p % m;
        n >>= 1;
    }
    ret
}

fn mod_div(a: usize, b: usize, m: usize) -> usize {
    (a * mod_pow(b, m - 2, m)) % m
}

fn prime_factorize(n: usize) -> BTreeMap<usize, usize> {
    let mut nn = n;
    let mut i = 2;
    let mut pf: BTreeMap<usize, usize> = BTreeMap::new();
    while i * i <= n {
        while nn % i == 0 {
            *pf.entry(i).or_default() += 1;
            nn /= i;
        }
        i += 1;
    }
    if nn != 1 {
        *pf.entry(nn).or_default() += 1;
    }
    pf
}
