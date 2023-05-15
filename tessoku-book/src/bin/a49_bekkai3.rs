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

const N: usize = 20;

macro_rules! input(($($tt:tt)*) => (
    let stdin = std::io::stdin();
    let mut stdin = proconio::source::line::LineSource::new(stdin.lock());
    proconio::input!(from &mut stdin, $($tt)*);
));

use lazy_static::lazy_static;

lazy_static! {
    static ref _INPUT: (usize, Vec<(usize, usize, usize)>) = {
        input! { t: usize, pqr: [(Usize1, Usize1, Usize1); t], }
        (t, pqr)
    };
    static ref T: usize = _INPUT.0;
    static ref PQR: Vec<(usize, usize, usize)> = _INPUT.1.clone();
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

#[derive(Debug, Clone, Eq)]
struct State {
    X: [i8; N],
    turn: usize,
    score: usize,
    last_action: bool,
    before_index: usize,
}

impl State {
    fn new() -> Self {
        State {
            X: [0; N],
            turn: 0,
            score: 0,
            last_action: true,
            before_index: 0,
        }
    }
    fn get_score(&self) -> usize {
        self.X.iter().filter(|&&x| x == 0).count()
    }
    fn advance(&mut self, action: bool) {
        let d = if action { 1 } else { -1 };
        let p = PQR[self.turn].0;
        let q = PQR[self.turn].1;
        let r = PQR[self.turn].2;

        self.X[p] += d;
        self.X[q] += d;
        self.X[r] += d;
        self.score += self.get_score();
        self.turn += 1;
    }
}

impl std::cmp::PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.score == other.score {
            Some(std::cmp::Ordering::Equal)
        } else if self.score < other.score {
            Some(std::cmp::Ordering::Greater)
        } else if self.score > other.score {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.score == other.score {
            std::cmp::Ordering::Equal
        } else if self.score < other.score {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }
}


#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        lazy_static::initialize(&_INPUT);

        let state = State::new();
        let start = std::time::Instant::now();

        let beam_width = 60000;
        let mut beam = vec![vec![]; *T + 1];
        beam[0].push(state);

        for t in 1..=*T {
            let mut candidate = vec![];

            for (i, now_state) in beam[t - 1].iter().enumerate() {
                for j in 0..2 {
                    let mut next_state = now_state.clone();
                    let action = j == 0;
                    next_state.advance(action);
                    next_state.last_action = action;
                    next_state.before_index = i;
                    candidate.push(next_state);
                }
            }
            candidate.sort();
            beam[t] = candidate[..min!(beam_width, candidate.len())].to_vec();
        }

        let mut ans = VecDeque::new();
        let mut before_index = 0;
        for t in (1..=*T).rev() {
            let last_action = beam[t][before_index].last_action;
            before_index = beam[t][before_index].before_index;
            ans.push_front(if last_action { 'A' } else { 'B' });
        }
        eprintln!("Score: {}", beam[*T][0].score);
        println!("{}", ans.iter().join(" "));

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

fn main() {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
