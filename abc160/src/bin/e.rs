#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use std::collections::BTreeMap;

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

const MOD: usize = 1e9 as usize + 7;
// const MOD: usize = 998244353;
// const MOD: usize = 2147483647;

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
            parent: vec![-1; n],
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
    fn roots(&self) -> Vec<usize> {
        (0..self.parent.len())
            .filter(|i| self.parent[*i] < 0)
            .collect::<Vec<usize>>()
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

#[derive(Debug, Clone)]
struct Comb {
    facts: Vec<usize>,
    fact_invs: Vec<usize>,
}

impl Comb {
    fn new(n: usize) -> Self {
        let mut facts = vec![1, 1];
        let mut fact_invs = vec![1, 1];
        let mut invs = vec![0, 1];
        for i in 2..=n {
            facts.push(facts.last().unwrap() * i % MOD);
            invs.push((MOD - invs[MOD % i]) * (MOD / i) % MOD);
            fact_invs.push(fact_invs.last().unwrap() * invs.last().unwrap() % MOD);
        }
        Comb { facts, fact_invs }
    }
    fn nCr(&self, n: usize, r: usize) -> usize {
        self.facts[n] * self.fact_invs[n - r] % MOD * self.fact_invs[r] % MOD
    }
    fn nHr(&self, n: usize, r: usize) -> usize {
        self.nCr(n + r - 1, r)
    }
}

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            X: usize,
            Y: usize,
            A: usize,
            B: usize,
            C: usize,
            mut P: [usize; A],
            mut Q: [usize; B],
            mut R: [usize; C]
        }

        P.sort();
        P.reverse();
        Q.sort();
        Q.reverse();
        R.sort();
        R.reverse();

        let mut PQ = vec![];
        for i in 0..X {
            PQ.push(P[i]);
        }
        for i in 0..Y {
            PQ.push(Q[i]);
        }
        PQ.sort();
        PQ.reverse();

        let mut S_PQ = vec![0_usize; X + Y + 1];
        for i in 1..X + Y + 1 {
            S_PQ[i] = S_PQ[i - 1] + PQ[i - 1];
        }
        let mut S_R = vec![0_usize; C + 1];
        for i in 1..C + 1 {
            S_R[i] = S_R[i - 1] + R[i - 1];
        }

        let mut ans = 0_usize;
        for c in 0..min!(C, X + Y) + 1 {
            ans = max!(ans, S_R[c] + S_PQ[X + Y - c]);
        }
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

fn legendre(n: usize, p: usize) -> usize {
    let mut cnt = 0_usize;
    let mut pp = p;
    while pp <= n {
        cnt += n / pp;
        pp *= p;
    }
    cnt
}

fn mod_pow(a: usize, b: usize) -> usize {
    let mut p = a;
    let mut ret = 1;
    let mut n = b;
    while n > 0 {
        if n & 1 == 1 {
            ret = ret * p % MOD;
        }
        p = p * p % MOD;
        n >>= 1;
    }
    ret
}

fn mod_pow2(a: usize, b: usize, m: usize) -> usize {
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

fn mod_inv(a: usize, b: usize) -> usize {
    (a * mod_pow(b, MOD - 2)) % MOD
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

fn enum_dividers(n: usize) -> Vec<usize> {
    let mut i = 1_usize;
    let mut ret = vec![];
    while i * i <= n {
        if n % i == 0 {
            ret.push(i);
            if i != n / i {
                ret.push(n / i);
            }
        }
        i += 1;
    }
    ret.sort();
    ret
}

// ax+by=gcd(a, b)
fn ext_gcd(a: usize, b: usize) -> (isize, isize, usize) {
    if a == 0 {
        return (0, 1, b);
    }
    let (x, y, g) = ext_gcd(b % a, a);
    (y - b as isize / a as isize * x, x, g)
}

fn mod_inv2(x: usize) -> usize {
    (ext_gcd(x, MOD).0 + MOD as isize) as usize % MOD
}
