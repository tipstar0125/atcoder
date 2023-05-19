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
            mut T: Chars
        }

        let T_rev: Vec<char> = T.iter().cloned().rev().collect();
        T.extend(T_rev);
        let rh = RH::new(T.clone());
        for i in 0..N {
            if rh.connect(0, i, N + i, 2 * N).value() == rh.get(3 * N - i, 4 * N - i).value() {
                println!("{}", T[3 * N - i..4 * N - i].iter().join(""));
                println!("{}", i);
                return;
            }
        }
        println!("-1");
    }
}

const MOD: usize = 2147483647;

type RH = RollingHash;
struct RollingHash {
    base: ModInt,
    power: Vec<ModInt>,
    hash: Vec<ModInt>,
}

impl RollingHash {
    fn new(s: Vec<char>) -> Self {
        let n = s.len();
        let base = Mod::new(rnd::gen_range(2, 1000));

        let mut p = Mod::one();
        let mut power = vec![p];
        let mut h = Mod::zero();
        let mut hash = vec![h];

        for i in 0..n {
            p *= base;
            power.push(p);

            let num = Mod::new(s[i] as u8 as usize);
            h *= base;
            h += num;
            hash.push(h);
        }
        RollingHash { base, power, hash }
    }
    // [l, r)
    fn get(&self, l: usize, r: usize) -> ModInt {
        self.hash[r] - self.hash[l] * self.power[r - l]
    }
    fn connect(&self, l1: usize, r1: usize, l2: usize, r2: usize) -> ModInt {
        let h1 = self.get(l1, r1);
        let h2 = self.get(l2, r2);
        h1 * self.power[r2 - l2] + h2
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
        fn ext_gcd(a: usize, b: usize) -> (isize, isize, usize) {
            if a == 0 {
                return (0, 1, b);
            }
            let (x, y, g) = ext_gcd(b % a, a);
            (y - b as isize / a as isize * x, x, g)
        }

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

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
