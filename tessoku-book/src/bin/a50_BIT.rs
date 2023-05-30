#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

const N: usize = 100;
const MAX_SCORE: isize = 2e8 as isize;
const MAX_Q: usize = 1000;

use lazy_static::lazy_static;

macro_rules! input(($($tt:tt)*) => (
    let stdin = std::io::stdin();
    let mut stdin = proconio::source::line::LineSource::new(stdin.lock());
    proconio::input!(from &mut stdin, $($tt)*);
));

lazy_static! {
    static ref A: Vec<Vec<isize>> = {
        input! { a: [[isize; N]; N]}
        a
    };
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

#[derive(Debug, Clone)]
struct TimeKeeper {
    start_time: std::time::Instant,
    time_threshold: f64,
}

impl TimeKeeper {
    fn new(time_threshold: f64) -> Self {
        TimeKeeper {
            start_time: std::time::Instant::now(),
            time_threshold,
        }
    }
    #[inline]
    fn isTimeOver(&self) -> bool {
        let elapsed_time = self.start_time.elapsed().as_nanos() as f64 * 1e-9;
        #[cfg(feature = "local")]
        {
            elapsed_time * 1.5 >= self.time_threshold
        }
        #[cfg(not(feature = "local"))]
        {
            elapsed_time >= self.time_threshold
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    mountain: Vec<Vec<isize>>,
    turn: usize,
    score: isize,
}

impl State {
    fn new() -> Self {
        let mut score = MAX_SCORE;
        for i in 0..N {
            for j in 0..N {
                score -= A[i][j];
            }
        }

        State {
            mountain: vec![vec![0; N]; N],
            turn: 0,
            score,
        }
    }
    fn isDone(&self) -> bool {
        self.turn == MAX_Q
    }
    fn get_score(&self, x: usize, y: usize, h: usize) -> isize {
        let mut score = self.score;
        for i in 0..N {
            for j in 0..N {
                let manhattan_dist =
                    (i as isize - x as isize).abs() + (j as isize - y as isize).abs();
                if manhattan_dist >= h as isize {
                    continue;
                }
                let add_h = h as isize - manhattan_dist;
                let a = A[j][i];
                let mountain = self.mountain[j][i];
                let before_diff = if a <= mountain {
                    (mountain - a) * 2
                } else {
                    a - mountain
                };
                let now_diff = if a <= mountain + add_h {
                    (mountain + add_h - a) * 2
                } else {
                    a - mountain - add_h
                };
                score += before_diff - now_diff;
            }
        }
        score
    }
    fn advance(&mut self, x: usize, y: usize, h: usize, bit: &mut BinaryIndexedTree) {
        for i in 0..N {
            for j in 0..N {
                let manhattan_dist =
                    (i as isize - x as isize).abs() + (j as isize - y as isize).abs();
                if manhattan_dist >= h as isize {
                    continue;
                }
                let add_h = h as isize - manhattan_dist;
                self.mountain[j][i] += add_h;
                let delta = -min!(bit.range_sum(i * N + j, i * N + j + 1), add_h);
                bit.add(i * N + j, delta);
            }
        }
        self.turn += 1;
    }
}

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        lazy_static::initialize(&A);

        #[allow(unused_mut, unused_assignments)]
        let mut seed = 0;
        #[cfg(feature = "seed")]
        {
            seed = 11216848234635351618;
        }
        eprintln!("seed: {}", seed);
        rnd::init(seed);

        let start = std::time::Instant::now();
        let time_keeper = TimeKeeper::new(5.98);
        let mut state = State::new();
        let mut query = vec![];

        let get_point_lower_start = 10000;
        let get_point_lower_end = 0;

        let mut bit = BinaryIndexedTree::new(N * N);
        for i in 0..N {
            for j in 0..N {
                let pos = i * N + j;
                bit.add(pos, A[i][j]);
            }
        }
        let mut rnd_max = bit.sum(N * N) as usize;

        while !state.isDone() && !time_keeper.isTimeOver() {
            let r = rnd::gen_range(0, rnd_max);
            let pos = bit.upper_bound(r as isize);
            let y = pos / N;
            let x = pos % N;
            let h = rnd::gen_range(1, N + 1);

            let current_score = state.score;
            let new_score = state.get_score(x, y, h);

            let get_point_lower = get_point_lower_start
                - (get_point_lower_start - get_point_lower_end) * state.turn / MAX_Q;

            if new_score >= current_score + get_point_lower as isize {
                state.advance(x, y, h, &mut bit);
                state.score = new_score;
                query.push((x, y, h));
                rnd_max = bit.sum(N * N) as usize;
            }
        }

        println!("{}", query.len());
        for q in &query {
            println!("{} {} {}", q.0, q.1, q.2);
        }

        #[allow(unused_mut, unused_assignments)]
        let mut elapsed_time = start.elapsed().as_micros() as f64 * 1e-6;
        #[cfg(feature = "local")]
        {
            eprintln!("Local Mode");
            elapsed_time *= 1.5;
        }
        eprintln!("Elapsed time: {}sec", elapsed_time);
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
struct BinaryIndexedTree {
    size: usize,
    data: Vec<isize>,
}

impl BinaryIndexedTree {
    fn new(n: usize) -> Self {
        BinaryIndexedTree {
            size: n,
            data: vec![0; n],
        }
    }
    fn lsb(&self, i: usize) -> usize {
        i & i.wrapping_neg()
    }
    fn build(&mut self, v: &[isize]) {
        assert_eq!(self.size, v.len(), "size not correct!");
        self.data = v.to_vec();
        for i in 1..=self.size {
            let lsb = self.lsb(i);
            if i + lsb <= self.size {
                self.data[i + lsb - 1] += self.data[i - 1];
            }
        }
    }
    fn push(&mut self, mut x: isize) {
        self.size += 1;
        let mut d = 1;
        let k = self.lsb(self.size);
        while d != k {
            x += self.data[self.size - d - 1];
            d <<= 1;
        }
        self.data.push(x);
    }
    fn add(&mut self, i: usize, x: isize) {
        let mut idx = i + 1;
        while idx <= self.size {
            self.data[idx - 1] += x;
            idx += self.lsb(idx);
        }
    }
    //  [0, r)
    fn sum(&self, i: usize) -> isize {
        let mut ret = 0;
        let mut idx = i;
        while idx > 0 {
            ret += self.data[idx - 1];
            idx -= self.lsb(idx);
        }
        ret
    }
    // [l, r)
    fn range_sum(&self, l: usize, r: usize) -> isize {
        self.sum(r) - self.sum(l)
    }
    fn lower_bound(&self, x: isize) -> usize {
        let mut i = 0;
        let mut k = 1;
        let mut x = x;
        while k <= self.size {
            k <<= 1;
        }
        while k > 0 {
            if i + k <= self.size && self.data[i + k - 1] < x {
                x -= self.data[i + k - 1];
                i += k;
            }
            k >>= 1;
        }
        if x > 0 {
            i
        } else {
            0
        }
    }
    fn upper_bound(&self, x: isize) -> usize {
        let mut i = 0;
        let mut k = 1;
        let mut x = x;
        while k <= self.size {
            k <<= 1;
        }
        while k > 0 {
            if i + k <= self.size && self.data[i + k - 1] <= x {
                x -= self.data[i + k - 1];
                i += k;
            }
            k >>= 1;
        }
        if i < self.size {
            i
        } else {
            self.size
        }
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
