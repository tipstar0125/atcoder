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
    fn get_score(&self, x: usize, y: usize, h: usize, is_redo: bool) -> isize {
        let mut score = self.score;
        let penalty = 100;
        for i in 0..N {
            for j in 0..N {
                let manhattan_dist =
                    (i as isize - x as isize).abs() + (j as isize - y as isize).abs();
                if manhattan_dist >= h as isize {
                    continue;
                }
                let mut add_h = h as isize - manhattan_dist;
                if is_redo {
                    add_h *= -1;
                }
                let a = A[i][j];
                let mountain = self.mountain[i][j];
                let mut before_diff = (a - mountain).abs();
                if mountain >= a {
                    before_diff += penalty;
                }
                let mut now_diff = (a - (mountain + add_h)).abs();
                if mountain + add_h >= a {
                    now_diff += penalty;
                }
                score += before_diff - now_diff;
            }
        }
        score
    }
    fn advance(&mut self, x: usize, y: usize, h: usize, is_redo: bool) {
        for i in 0..N {
            for j in 0..N {
                let manhattan_dist =
                    (i as isize - x as isize).abs() + (j as isize - y as isize).abs();
                if manhattan_dist >= h as isize {
                    continue;
                }
                let mut add_h = h as isize - manhattan_dist;
                if is_redo {
                    add_h *= -1;
                }
                self.mountain[i][j] += add_h;
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

        while !state.isDone() && !time_keeper.isTimeOver() {
            let x = rnd::gen_range(0, N);
            let y = rnd::gen_range(0, N);
            let h = rnd::gen_range(1, N + 1);

            let current_score = state.score;
            let new_score = state.get_score(x, y, h, false);

            if new_score >= current_score {
                state.advance(x, y, h, false);
                query.push((x, y, h));
            }
        }

        let L = query.len();
        let mut is_removed = false;
        let mut removed_idx = L;

        while !time_keeper.isTimeOver() {
            if !is_removed {
                let idx = rnd::gen_range(0, L);
                let (x, y, h) = query[idx];
                let current_score = state.score;
                let new_score = state.get_score(x, y, h, true);
                if new_score >= current_score {
                    state.advance(x, y, h, true);
                    removed_idx = idx;
                    is_removed = true;
                }
            } else {
                let x = rnd::gen_range(0, N);
                let y = rnd::gen_range(0, N);
                let h = rnd::gen_range(1, N + 1);

                let current_score = state.score;
                let new_score = state.get_score(x, y, h, false);

                if new_score >= current_score {
                    state.advance(x, y, h, false);
                    query[removed_idx] = (x, y, h);
                    is_removed = false;
                }
            }
        }

        println!("{}", L);
        for i in 0..L {
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
