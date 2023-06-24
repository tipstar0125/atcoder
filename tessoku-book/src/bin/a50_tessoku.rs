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
    pub fn gen_range_isize(a: usize) -> isize {
        let mut x = (gen() % a) as isize;
        if gen_bool() {
            x *= -1;
        }
        x
    }
    #[inline]
    pub fn gen_range_neg_wrapping(a: usize) -> usize {
        let mut x = gen() % a;
        if gen_bool() {
            x = x.wrapping_neg();
        }
        x
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
    #[inline]
    fn get_time(&self) -> f64 {
        let elapsed_time = self.start_time.elapsed().as_nanos() as f64 * 1e-9;
        #[cfg(feature = "local")]
        {
            elapsed_time * 1.5
        }
        #[cfg(not(feature = "local"))]
        {
            elapsed_time
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    B: Vec<Vec<isize>>,
    turn: usize,
    score: isize,
}

impl State {
    fn new() -> Self {
        State {
            B: vec![vec![0; N]; N],
            turn: 0,
            score: 0,
        }
    }
    fn is_legal_action(&self, x: usize, y: usize, h: usize) -> bool {
        x < N && y < N && (1..=N).contains(&h)
    }
    fn get_score(&self) -> isize {
        let mut score = MAX_SCORE;
        for i in 0..N {
            for j in 0..N {
                score -= (A[i][j] - self.B[i][j]).abs();
            }
        }
        score
    }
    fn change(&mut self, query0: (usize, usize, usize), query1: (usize, usize, usize)) {
        let (x0, y0, h0) = query0;
        let (x1, y1, h1) = query1;

        for i in 0..N {
            for j in 0..N {
                let manhattan_dist0 =
                    (i as isize - x0 as isize).abs() + (j as isize - y0 as isize).abs();
                if manhattan_dist0 < h0 as isize {
                    let sub_h = h0 as isize - manhattan_dist0;
                    self.B[j][i] -= sub_h;
                }
            }
        }

        for i in 0..N {
            for j in 0..N {
                let manhattan_dist1 =
                    (i as isize - x1 as isize).abs() + (j as isize - y1 as isize).abs();
                if manhattan_dist1 < h1 as isize {
                    let add_h = h1 as isize - manhattan_dist1;
                    self.B[j][i] += add_h;
                }
            }
        }
    }
}

fn init(state: &mut State, query: &mut [(usize, usize, usize)]) {
    for i in 0..MAX_Q {
        let x = rnd::gen_range(0, N);
        let y = rnd::gen_range(0, N);
        let h = 1;
        assert!(state.is_legal_action(x, y, h));
        state.change(query[i], (x, y, h));
        query[i] = (x, y, h);
    }
    state.score = state.get_score();
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
        let mut query = vec![(0, 0, 0); MAX_Q];

        init(&mut state, &mut query);
        let mut cnt = 0;

        while !time_keeper.isTimeOver() {
            let idx = rnd::gen_range(0, MAX_Q);
            let (x0, y0, h0) = query[idx];
            let dx = rnd::gen_range_neg_wrapping(10);
            let dy = rnd::gen_range_neg_wrapping(10);
            let dh = rnd::gen_range_neg_wrapping(20);
            let x = x0.wrapping_add(dx);
            let y = y0.wrapping_add(dy);
            let h = h0.wrapping_add(dh);
            if !state.is_legal_action(x, y, h) {
                continue;
            }
            let current_score = state.score;
            state.change(query[idx], (x, y, h));
            let new_score = state.get_score();
            if new_score > current_score {
                query[idx] = (x, y, h);
                state.score = new_score;
                cnt += 1;
            } else {
                state.change((x, y, h), query[idx]);
            }
        }

        eprintln!("Update count: {}", cnt);
        eprintln!("Score: {}", state.score);

        println!("{}", MAX_Q);
        for i in 0..MAX_Q {
            let (x, y, h) = query[i];
            println!("{} {} {}", x, y, h);
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

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}