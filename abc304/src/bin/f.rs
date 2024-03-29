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
            S: Chars
        }

        let E = Eratosthenes::new(N);
        let mut ans = Mod::zero();

        for &t in &E.dividers(N) {
            if t == N {
                break;
            }
            let mut pos = 0_usize;
            let mut v = vec![true; t];
            while pos < N {
                let s = &S[pos..pos + t];
                for (i, si) in s.iter().enumerate() {
                    if *si == '.' {
                        v[i] = false;
                    }
                }
                pos += t;
            }
            let exp = v.iter().filter(|&&x| x).count();
            let cnt = Mod::new(2).pow(exp);
            if E.mobius[N / t] == 1 {
                ans -= cnt;
            } else if E.mobius[N / t] == -1 {
                ans += cnt;
            }
        }
        println!("{}", ans.value());
    }
}

#[derive(Debug, Clone, Default)]
struct Eratosthenes {
    is_prime: Vec<bool>,
    min_factor: Vec<isize>,
    mobius: Vec<isize>,
}

impl Eratosthenes {
    fn new(n: usize) -> Self {
        let mut is_prime = vec![true; n + 1];
        let mut min_factor = vec![-1; n + 1];
        let mut mobius = vec![1; n + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        min_factor[1] = 1;

        for p in 2..=n {
            if !is_prime[p] {
                continue;
            }
            min_factor[p] = p as isize;
            mobius[p] = -1;

            let mut q = p * 2;
            while q <= n {
                is_prime[q] = false;
                if min_factor[q] == -1 {
                    min_factor[q] = p as isize;
                }
                if (q / p) % p == 0 {
                    mobius[q] = 0;
                } else {
                    mobius[q] = -mobius[q];
                }
                q += p;
            }
        }
        Eratosthenes {
            is_prime,
            min_factor,
            mobius,
        }
    }
    fn factorize(&self, N: usize) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        let mut n = N;
        while n > 1 {
            let p = self.min_factor[n];
            let mut exp = 0;

            while self.min_factor[n] == p {
                n /= p as usize;
                exp += 1;
            }
            ret.push((p as usize, exp));
        }
        ret
    }
    fn dividers(&self, N: usize) -> Vec<usize> {
        let mut ret = vec![1];
        let pf = self.factorize(N);
        for &(p, exp) in &pf {
            let m = ret.len();
            for i in 0..m {
                let mut v = 1;
                for _ in 0..exp {
                    v *= p;
                    ret.push(ret[i] * v);
                }
            }
        }
        ret.sort();
        ret
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
