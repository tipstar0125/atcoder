#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

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
    static ref T: usize = {
        input! { t: usize }
        t
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

#[derive(Debug, Clone, Eq)]
struct State {
    PQR: Vec<(usize, usize, usize)>,
    X: [isize; N],
    turn: usize,
    score: usize,
    evaluated_score: usize,
    fist_action: bool,
    actions: Vec<char>,
}

impl State {
    fn new() -> Self {
        input! {
            PQR: [(Usize1, Usize1, Usize1); *T]
        }

        State {
            PQR,
            X: [0; N],
            turn: 0,
            score: 0,
            evaluated_score: 0,
            fist_action: true,
            actions: vec![],
        }
    }
    fn get_score(&self) -> usize {
        self.X.iter().fold(0, |sum, &x| sum + x.abs()) as usize
    }
    fn isDone(&self) -> bool {
        self.turn == *T
    }
    fn advance(&mut self, action: bool) {
        let d = if action { 1 } else { -1 };

        let p = self.PQR[self.turn].0;
        let q = self.PQR[self.turn].1;
        let r = self.PQR[self.turn].2;
        self.X[p] += d;
        self.X[q] += d;
        self.X[r] += d;
        self.score += self.get_score();

        self.turn += 1;
        if action {
            self.actions.push('A');
        } else {
            self.actions.push('B');
        }
    }
    fn evaluate_score(&mut self) {
        self.evaluated_score = self.score;
    }
}

impl std::cmp::PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.evaluated_score == other.evaluated_score
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.evaluated_score == other.evaluated_score {
            Some(std::cmp::Ordering::Equal)
        } else if self.evaluated_score < other.evaluated_score {
            Some(std::cmp::Ordering::Greater)
        } else if self.evaluated_score > other.evaluated_score {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.evaluated_score == other.evaluated_score {
            std::cmp::Ordering::Equal
        } else if self.evaluated_score < other.evaluated_score {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }
}

fn random_action() -> bool {
    rnd::gen_bool()
}

fn greedy_action(state: &State) -> bool {
    let mut state_A = state.clone();
    let mut state_B = state.clone();
    state_A.advance(true);
    state_A.evaluate_score();
    state_B.advance(false);
    state_B.evaluate_score();
    state_A > state_B
}

fn beam_search_action(state: &State, beam_width: usize, time_threshold: f64) -> bool {
    let mut now_beam = BinaryHeap::new();
    let mut best_state = state;
    now_beam.push(state.clone());
    let time_keeper = TimeKeeper::new(time_threshold);

    for t in 0.. {
        let mut next_beam = BinaryHeap::new();
        for _ in 0..beam_width {
            if now_beam.is_empty() {
                break;
            }
            let now_state = now_beam.pop().unwrap();
            let mut next_state_A = now_state.clone();
            let mut next_state_B = now_state.clone();
            next_state_A.advance(true);
            next_state_A.evaluate_score();
            next_state_B.advance(false);
            next_state_B.evaluate_score();
            if t == 0 {
                next_state_A.fist_action = true;
                next_state_B.fist_action = false;
            }
            next_beam.push(next_state_A);
            next_beam.push(next_state_B);
        }
        now_beam = next_beam;
        best_state = now_beam.peek().unwrap();
        if best_state.isDone() || time_keeper.isTimeOver() {
            break;
        }
    }
    best_state.fist_action
}

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        lazy_static::initialize(&T);

        let mut state = State::new();
        let start = std::time::Instant::now();
        let LIMIT = 1000; // [msec]
        let time_threshold = (LIMIT / *T / 10 * 4) as f64 * 1e-3; // [sec]

        while !state.isDone() {
            // state.advance(random_action());
            // state.advance(greedy_action(&state));
            state.advance(beam_search_action(&state, 10000, time_threshold));
        }
        println!("{}", state.actions.iter().join(" "));

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
