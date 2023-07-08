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
            N1: usize,
            N2: usize,
            M: usize,
            AB: [(Usize1, Usize1); M]
        }

        let mut uf = UnionFind::new(N1 + N2);
        for &(a, b) in &AB {
            uf.unite(a, b);
        }

        let all_group_members = uf.all_group_members();
        let keys: Vec<_> = all_group_members.keys().collect();
        let key1 = keys[0];
        let key2 = keys[1];
        let mut A: BTreeSet<usize> = BTreeSet::new();
        let mut B: BTreeSet<usize> = BTreeSet::new();
        if all_group_members[key1].len() == N1 {
            A = all_group_members[key1].clone().into_iter().collect();
            B = all_group_members[key2].clone().into_iter().collect();
        } else {
            B = all_group_members[key1].clone().into_iter().collect();
            A = all_group_members[key2].clone().into_iter().collect();
        }

        let mut G1 = vec![vec![]; N1 + N2];
        let mut G2 = vec![vec![]; N1 + N2];
        for &(a, b) in &AB {
            if A.contains(&a) {
                G1[a].push(b);
                G1[b].push(a);
            } else {
                G2[a].push(b);
                G2[b].push(a);
            }
        }

        let INF = 1_usize << 60;
        let mut dist1 = vec![INF; N1 + N2];
        let mut Q = VecDeque::new();
        if A.contains(&0) {
            Q.push_back(0);
            dist1[0] = 0;
        } else {
            Q.push_back(N1 + N2 - 1);
            dist1[N1 + N2 - 1] = 0;
        }
        while !Q.is_empty() {
            let pos = Q.pop_front().unwrap();
            for &next in &G1[pos] {
                if dist1[next] == INF {
                    dist1[next] = dist1[pos] + 1;
                    Q.push_back(next);
                }
            }
        }


        let mut dist2 = vec![INF; N1 + N2];
        let mut Q = VecDeque::new();
        if B.contains(&0) {
            Q.push_back(0);
            dist2[0] = 0;
        } else {
            Q.push_back(N1 + N2 - 1);
            dist2[N1 + N2 - 1] = 0;
        }
        while !Q.is_empty() {
            let pos = Q.pop_front().unwrap();
            for &next in &G2[pos] {
                if dist2[next] == INF {
                    dist2[next] = dist2[pos] + 1;
                    Q.push_back(next);
                }
            }
        }
        let ans = dist1.iter().filter(|&&x| x < INF).max().unwrap()
            + dist2.iter().filter(|&&x| x < INF).max().unwrap()
            + 1;
        println!("{}", ans);
    }
}

#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<isize>,
    roots: BTreeSet<usize>,
    size: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut roots = BTreeSet::new();
        for i in 0..n {
            roots.insert(i);
        }
        UnionFind {
            parent: vec![-1; n],
            roots,
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
            self.roots.remove(&root_y);
            Some((root_x, root_y))
        } else {
            self.parent[root_y] -= size_x;
            self.parent[root_x] = root_y as isize;
            self.roots.remove(&root_x);
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
    fn members(&mut self, x: usize) -> Vec<usize> {
        let root = self.find(x);
        (0..self.parent.len())
            .filter(|i| self.find(*i) == root)
            .collect::<Vec<usize>>()
    }
    fn all_group_members(&mut self) -> BTreeMap<usize, Vec<usize>> {
        let mut groups_map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for x in 0..self.parent.len() {
            let r = self.find(x);
            groups_map.entry(r).or_default().push(x);
        }
        groups_map
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
