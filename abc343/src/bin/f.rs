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
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque};
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
            Q: usize,
            A: [usize; N]
        }
        let mut seg = Segtree::<MaxMonoid>::new(N);
        for (i, &a) in A.iter().enumerate() {
            seg.set(i, ((0, 0), (a, 1)));
        }
        for _ in 0..Q {
            input! {
                t: usize
            }
            if t == 1 {
                input! {
                    p: Usize1,
                    x: usize
                }
                seg.set(p, ((0, 0), (x, 1)));
            } else {
                input! {
                    l: Usize1,
                    r: Usize1
                }
                let ans = seg.prod(l..=r);
                println!("{}", ans.0 .1);
            }
        }
    }
}

fn ceil_pow2(n: u32) -> u32 {
    32 - n.saturating_sub(1).leading_zeros()
}

pub trait Integral:
    'static
    + Send
    + Sync
    + Copy
    + Ord
    + std::ops::Not<Output = Self>
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Rem<Output = Self>
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + std::ops::DivAssign
    + std::ops::RemAssign
    + std::iter::Sum
    + std::iter::Product
    + std::ops::BitOr<Output = Self>
    + std::ops::BitAnd<Output = Self>
    + std::ops::BitXor<Output = Self>
    + std::ops::BitOrAssign
    + std::ops::BitAndAssign
    + std::ops::BitXorAssign
    + std::ops::Shl<Output = Self>
    + std::ops::Shr<Output = Self>
    + std::ops::ShlAssign
    + std::ops::ShrAssign
    + std::fmt::Display
    + std::fmt::Debug
    + std::fmt::Binary
    + std::fmt::Octal
    + Zero
    + One
    + BoundedBelow
    + BoundedAbove
{
}

/// Class that has additive identity element
pub trait Zero {
    /// The additive identity element
    fn zero() -> Self;
}

/// Class that has multiplicative identity element
pub trait One {
    /// The multiplicative identity element
    fn one() -> Self;
}

pub trait BoundedBelow {
    fn min_value() -> Self;
}

pub trait BoundedAbove {
    fn max_value() -> Self;
}

macro_rules! impl_integral {
    ($($ty:ty),*) => {
        $(
            impl Zero for $ty {
                #[inline]
                fn zero() -> Self {
                    0
                }
            }

            impl One for $ty {
                #[inline]
                fn one() -> Self {
                    1
                }
            }

            impl BoundedBelow for $ty {
                #[inline]
                fn min_value() -> Self {
                    Self::min_value()
                }
            }

            impl BoundedAbove for $ty {
                #[inline]
                fn max_value() -> Self {
                    Self::max_value()
                }
            }

            impl Integral for $ty {}
        )*
    };
}

impl_integral!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

pub trait Monoid {
    type S: Clone;
    fn identity() -> Self::S;
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;
}

pub struct Max<S>(
    std::convert::Infallible,
    std::marker::PhantomData<fn() -> S>,
);
impl<S> Monoid for Max<S>
where
    S: Copy + Ord + BoundedBelow,
{
    type S = S;
    fn identity() -> Self::S {
        S::min_value()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        std::cmp::max(*a, *b)
    }
}

struct MaxMonoid;

impl Monoid for MaxMonoid {
    type S = ((usize, usize), (usize, usize));
    fn identity() -> Self::S {
        ((0, 0), (0, 0))
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        let x = a.0 .0;
        let cnt_x = a.0 .1;
        let y = a.1 .0;
        let cnt_y = a.1 .1;
        let mut mp: BTreeMap<usize, usize> = BTreeMap::new();
        *mp.entry(x).or_default() += cnt_x;
        *mp.entry(y).or_default() += cnt_y;

        let x = b.0 .0;
        let cnt_x = b.0 .1;
        let y = b.1 .0;
        let cnt_y = b.1 .1;
        *mp.entry(x).or_default() += cnt_x;
        *mp.entry(y).or_default() += cnt_y;

        let mut ret = vec![(0, 0)];
        for (&k, &v) in mp.iter() {
            ret.push((k, v));
        }
        (ret[ret.len() - 2], ret[ret.len() - 1])
    }
}

pub struct Min<S>(
    std::convert::Infallible,
    std::marker::PhantomData<fn() -> S>,
);
impl<S> Monoid for Min<S>
where
    S: Copy + Ord + BoundedAbove,
{
    type S = S;
    fn identity() -> Self::S {
        S::max_value()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        std::cmp::min(*a, *b)
    }
}

pub struct Additive<S>(
    std::convert::Infallible,
    std::marker::PhantomData<fn() -> S>,
);
impl<S> Monoid for Additive<S>
where
    S: Copy + std::ops::Add<Output = S> + Zero,
{
    type S = S;
    fn identity() -> Self::S {
        S::zero()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a + *b
    }
}

pub struct Multiplicative<S>(
    std::convert::Infallible,
    std::marker::PhantomData<fn() -> S>,
);
impl<S> Monoid for Multiplicative<S>
where
    S: Copy + std::ops::Mul<Output = S> + One,
{
    type S = S;
    fn identity() -> Self::S {
        S::one()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a * *b
    }
}

pub struct BitwiseOr<S>(
    std::convert::Infallible,
    std::marker::PhantomData<fn() -> S>,
);
impl<S> Monoid for BitwiseOr<S>
where
    S: Copy + std::ops::BitOr<Output = S> + Zero,
{
    type S = S;
    fn identity() -> Self::S {
        S::zero()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a | *b
    }
}

pub struct BitwiseAnd<S>(
    std::convert::Infallible,
    std::marker::PhantomData<fn() -> S>,
);
impl<S> Monoid for BitwiseAnd<S>
where
    S: Copy + std::ops::BitAnd<Output = S> + std::ops::Not<Output = S> + Zero,
{
    type S = S;
    fn identity() -> Self::S {
        !S::zero()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a & *b
    }
}

pub struct BitwiseXor<S>(
    std::convert::Infallible,
    std::marker::PhantomData<fn() -> S>,
);
impl<S> Monoid for BitwiseXor<S>
where
    S: Copy + std::ops::BitXor<Output = S> + Zero,
{
    type S = S;
    fn identity() -> Self::S {
        S::zero()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a ^ *b
    }
}

impl<M: Monoid> Default for Segtree<M> {
    fn default() -> Self {
        Segtree::new(0)
    }
}
impl<M: Monoid> Segtree<M> {
    pub fn new(n: usize) -> Segtree<M> {
        vec![M::identity(); n].into()
    }
}
impl<M: Monoid> From<Vec<M::S>> for Segtree<M> {
    fn from(v: Vec<M::S>) -> Self {
        let n = v.len();
        let log = ceil_pow2(n as u32) as usize;
        let size = 1 << log;
        let mut d = vec![M::identity(); 2 * size];
        d[size..][..n].clone_from_slice(&v);
        let mut ret = Segtree { n, size, log, d };
        for i in (1..size).rev() {
            ret.update(i);
        }
        ret
    }
}
impl<M: Monoid> FromIterator<M::S> for Segtree<M> {
    fn from_iter<T: IntoIterator<Item = M::S>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let n = iter.size_hint().0;
        let log = ceil_pow2(n as u32) as usize;
        let size = 1 << log;
        let mut d = Vec::with_capacity(size * 2);
        d.extend(
            std::iter::repeat_with(M::identity)
                .take(size)
                .chain(iter)
                .chain(std::iter::repeat_with(M::identity).take(size - n)),
        );
        let mut ret = Segtree { n, size, log, d };
        for i in (1..size).rev() {
            ret.update(i);
        }
        ret
    }
}
impl<M: Monoid> Segtree<M> {
    pub fn set(&mut self, mut p: usize, x: M::S) {
        assert!(p < self.n);
        p += self.size;
        self.d[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn get(&self, p: usize) -> M::S {
        assert!(p < self.n);
        self.d[p + self.size].clone()
    }

    pub fn get_slice(&self) -> &[M::S] {
        &self.d[self.size..][..self.n]
    }

    pub fn prod<R>(&self, range: R) -> M::S
    where
        R: std::ops::RangeBounds<usize>,
    {
        // Trivial optimization
        if range.start_bound() == std::ops::Bound::Unbounded
            && range.end_bound() == std::ops::Bound::Unbounded
        {
            return self.all_prod();
        }

        let mut r = match range.end_bound() {
            std::ops::Bound::Included(r) => r + 1,
            std::ops::Bound::Excluded(r) => *r,
            std::ops::Bound::Unbounded => self.n,
        };
        let mut l = match range.start_bound() {
            std::ops::Bound::Included(l) => *l,
            std::ops::Bound::Excluded(l) => l + 1,
            // TODO: There are another way of optimizing [0..r)
            std::ops::Bound::Unbounded => 0,
        };

        assert!(l <= r && r <= self.n);
        let mut sml = M::identity();
        let mut smr = M::identity();
        l += self.size;
        r += self.size;

        while l < r {
            if l & 1 != 0 {
                sml = M::binary_operation(&sml, &self.d[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = M::binary_operation(&self.d[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }

        M::binary_operation(&sml, &smr)
    }

    pub fn all_prod(&self) -> M::S {
        self.d[1].clone()
    }

    pub fn max_right<F>(&self, mut l: usize, f: F) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        assert!(l <= self.n);
        assert!(f(&M::identity()));
        if l == self.n {
            return self.n;
        }
        l += self.size;
        let mut sm = M::identity();
        while {
            // do
            while l % 2 == 0 {
                l >>= 1;
            }
            if !f(&M::binary_operation(&sm, &self.d[l])) {
                while l < self.size {
                    l *= 2;
                    let res = M::binary_operation(&sm, &self.d[l]);
                    if f(&res) {
                        sm = res;
                        l += 1;
                    }
                }
                return l - self.size;
            }
            sm = M::binary_operation(&sm, &self.d[l]);
            l += 1;
            // while
            {
                let l = l as isize;
                (l & -l) != l
            }
        } {}
        self.n
    }

    pub fn min_left<F>(&self, mut r: usize, f: F) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        assert!(r <= self.n);
        assert!(f(&M::identity()));
        if r == 0 {
            return 0;
        }
        r += self.size;
        let mut sm = M::identity();
        while {
            // do
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r >>= 1;
            }
            if !f(&M::binary_operation(&self.d[r], &sm)) {
                while r < self.size {
                    r = 2 * r + 1;
                    let res = M::binary_operation(&self.d[r], &sm);
                    if f(&res) {
                        sm = res;
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            sm = M::binary_operation(&self.d[r], &sm);
            // while
            {
                let r = r as isize;
                (r & -r) != r
            }
        } {}
        0
    }

    fn update(&mut self, k: usize) {
        self.d[k] = M::binary_operation(&self.d[2 * k], &self.d[2 * k + 1]);
    }
}

pub struct Segtree<M>
where
    M: Monoid,
{
    // variable name is _n in original library
    n: usize,
    size: usize,
    log: usize,
    d: Vec<M::S>,
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
