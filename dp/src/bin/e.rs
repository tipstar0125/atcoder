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

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

const MOD: usize = 1e9 as usize + 7;
// const MOD: usize = 998244353;
// const MOD: usize = 2147483647;

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {}
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

mod rnd {
    static mut S: usize = 0;
    static MAX: usize = 1e9 as usize;

    #[inline]
    pub fn init(seed: usize) {
        unsafe {
            if seed == 0 {
                let t = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as usize;
                S = t
            } else {
                S = seed;
            }
        }
    }
    #[inline]
    pub fn gen() -> usize {
        unsafe {
            if S == 0 {
                init(0);
            }
            S ^= S << 7;
            S ^= S >> 9;
            S
        }
    }
    #[inline]
    pub fn gen_range(a: usize, b: usize) -> usize {
        gen() % (b - a) + a
    }
    #[inline]
    pub fn gen_bool() -> bool {
        gen() & 1 == 1
    }
    #[inline]
    pub fn gen_float() -> f64 {
        ((gen() % MAX) as f64) / MAX as f64
    }
}

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
struct WeightedUnionFind {
    parent: Vec<isize>,
    size: usize,
    diff_weight: Vec<isize>,
}

impl WeightedUnionFind {
    fn new(n: usize) -> Self {
        WeightedUnionFind {
            parent: vec![-1; n],
            size: n,
            diff_weight: vec![0_isize; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            return x;
        }
        let root = self.find(self.parent[x] as usize);
        self.diff_weight[x] += self.diff_weight[self.parent[x] as usize];
        self.parent[x] = root as isize;
        root
    }
    fn weight(&mut self, x: usize) -> isize {
        self.find(x);
        self.diff_weight[x]
    }
    fn unite(&mut self, x: usize, y: usize, w: isize) -> Option<(usize, usize)> {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return None;
        }

        let adjusted_w = w + self.weight(x) - self.weight(y);
        let size_x = -self.parent[root_x];
        let size_y = -self.parent[root_y];
        self.size -= 1;
        if size_x >= size_y {
            self.diff_weight[root_y] = adjusted_w;
            self.parent[root_x] -= size_y;
            self.parent[root_y] = root_x as isize;
            Some((root_x, root_y))
        } else {
            self.diff_weight[root_x] = -adjusted_w;
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
    fn diff(&mut self, x: usize, y: usize) -> isize {
        self.weight(y) - self.weight(x)
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

type Mod = ModInt;
#[derive(Debug, Clone, Copy, Default)]
struct ModInt {
    value: usize,
}

impl ModInt {
    fn new(n: usize) -> Self {
        ModInt { value: n % MOD }
    }
    fn zero() -> Self {
        ModInt { value: 0 }
    }
    fn one() -> Self {
        ModInt { value: 1 }
    }
    fn value(&self) -> usize {
        self.value
    }
    fn pow(&self, n: usize) -> Self {
        let mut p = *self;
        let mut ret = ModInt::one();
        let mut nn = n;
        while nn > 0 {
            if nn & 1 == 1 {
                ret *= p;
            }
            p *= p;
            nn >>= 1;
        }
        ret
    }
    fn inv(&self) -> Self {
        ModInt::new((ext_gcd(self.value, MOD).0 + MOD as isize) as usize)
    }
}

impl std::ops::Add for ModInt {
    type Output = ModInt;
    fn add(self, other: Self) -> Self {
        ModInt::new(self.value + other.value)
    }
}

impl std::ops::Sub for ModInt {
    type Output = ModInt;
    fn sub(self, other: Self) -> Self {
        ModInt::new(MOD + self.value - other.value)
    }
}

impl std::ops::Mul for ModInt {
    type Output = ModInt;
    fn mul(self, other: Self) -> Self {
        ModInt::new(self.value * other.value)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl std::ops::Div for ModInt {
    type Output = ModInt;
    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}

impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl std::ops::DivAssign for ModInt {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

#[derive(Debug, Clone)]
struct Comb {
    fact: Vec<ModInt>,
    fact_inverse: Vec<ModInt>,
}

impl Comb {
    fn new(n: usize) -> Self {
        let mut fact = vec![Mod::one(), Mod::one()];
        let mut fact_inverse = vec![Mod::one(), Mod::one()];
        let mut inverse = vec![Mod::zero(), Mod::one()];
        for i in 2..=n {
            fact.push(*fact.last().unwrap() * Mod::new(i));
            inverse.push((Mod::zero() - inverse[MOD % i]) * Mod::new(MOD / i));
            fact_inverse.push(*fact_inverse.last().unwrap() * *inverse.last().unwrap());
        }
        Comb { fact, fact_inverse }
    }
    fn nCr(&self, n: usize, r: usize) -> ModInt {
        self.fact[n] * self.fact_inverse[n - r] * self.fact_inverse[r]
    }
    fn nHr(&self, n: usize, r: usize) -> ModInt {
        self.nCr(n + r - 1, r)
    }
}

trait ArgOrd<T> {
    fn argmax(&self) -> Option<usize>;
    fn argmin(&self) -> Option<usize>;
    fn argsort(&self) -> Vec<usize>;
    fn argsort_reverse(&self) -> Vec<usize>;
}

impl<T: Ord> ArgOrd<T> for [T] {
    fn argmax(&self) -> Option<usize> {
        (0..self.len()).max_by_key(|&i| &self[i])
    }

    fn argmin(&self) -> Option<usize> {
        (0..self.len()).min_by_key(|&i| &self[i])
    }
    fn argsort(&self) -> Vec<usize> {
        (0..self.len()).sorted_by_key(|&i| &self[i]).collect_vec()
    }
    fn argsort_reverse(&self) -> Vec<usize> {
        (0..self.len())
            .sorted_by_key(|&i| std::cmp::Reverse(&self[i]))
            .collect_vec()
    }
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

fn coordinate_compression<T: std::cmp::Ord + Copy>(v: Vec<T>) -> BTreeMap<T, usize> {
    let mut vv = v;
    vv.sort();
    vv.dedup();
    let ret = vv.iter().enumerate().map(|(i, &s)| (s, i)).collect();
    ret
}

fn transpose_vec<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let N = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..N)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn transpose_vec_deque<T>(v: VecDeque<VecDeque<T>>) -> VecDeque<VecDeque<T>> {
    assert!(!v.is_empty());
    let N = v[0].len();
    let mut iters: VecDeque<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..N)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<VecDeque<T>>()
        })
        .collect()
}

fn run_length_encoding<T: Eq>(v: Vec<T>) -> Vec<(T, usize)> {
    let mut v = v.into_iter().map(|v| (v, 1)).collect::<Vec<_>>();
    v.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });
    v
}
