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

const MOD: usize = 998244353;

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize,
            P: usize
        }
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

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
