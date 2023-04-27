#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

fn get_time() -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        #[cfg(feature = "local")]
        {
            (ms - STIME) * 1.5
        }
        #[cfg(not(feature = "local"))]
        {
            ms - STIME
        }
    }
}

const N: usize = 7;

#[derive(Debug)]
struct State {
    edge: [[(f64, usize); N - 1]; N],
    dist: [[f64; N]; N],
    route: [usize; N + 1],
    pos: [usize; N],
}

impl State {
    fn new() -> Self {
        input! {
            n: usize,
            XY: [(usize, usize); n]
        }

        let mut dist = [[0.; N]; N];
        let mut edge = [[(0., !0); N - 1]; N];
        let mut route = [[(0., !0); N - 1]; N];
        let mut route = [!0; N + 1];
        let mut pos = [!0; N];
        State {
            edge,
            dist,
            route,
            pos,
        }
    }
}

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        get_time();
        let mut state = State::new();
        eprintln!("{:?}", state);
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
