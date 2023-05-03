#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]

use std::collections::VecDeque;

use itertools::Itertools;
use ordered_float::NotNan;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[inline]
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

fn get_dist((x0, y0): (isize, isize), (x1, y1): (isize, isize)) -> f64 {
    let dx = x0 - x1;
    let dy = y0 - y1;
    ((dx * dx + dy * dy) as f64).sqrt()
}

#[derive(Debug)]
struct State {
    size: usize,
    edge: Vec<Vec<(f64, usize)>>,
    dist: Vec<Vec<f64>>,
    route: Vec<usize>,
    best_route: Vec<usize>,
    best_score: f64,
    idx: Vec<usize>,
}

impl State {
    fn new() -> Self {
        input! {
            N: usize,
            XY: [(isize, isize); N]
        }

        let mut dist = vec![vec![0.; N]; N];
        let mut edge = vec![vec![]; N];

        for i in 0..N {
            for j in 0..N {
                let d = get_dist(XY[i], XY[j]);
                dist[i][j] = d;
                if i != j {
                    edge[i].push((d, j));
                }
            }
            edge[i].sort_by(|&a, b| a.partial_cmp(b).unwrap());
        }

        // initialize by greedy
        let mut route = vec![];
        let mut visited = vec![false; N];
        let start = 0;
        let mut now = start;
        route.push(start);
        visited[start] = true;
        for _ in 0..N - 1 {
            for i in 0..N - 1 {
                let (_, next) = edge[now][i];
                if !visited[next] {
                    visited[next] = true;
                    route.push(next);
                    now = next;
                    break;
                }
            }
        }
        // route.push(0);

        let best_route = route.clone();
        let best_score = (0..N + 1)
            .collect_vec()
            .windows(2)
            .map(|w| dist[route[w[0] % N]][route[w[1] % N]])
            .sum::<f64>();

        let mut idx = vec![0; N];
        for i in 0..N {
            idx[route[i]] = i;
        }

        State {
            size: N,
            edge,
            dist,
            route,
            best_route,
            best_score,
            idx,
        }
    }
    #[inline]
    fn get_score(&self, route: &[usize]) -> f64 {
        let N = self.size;
        // let mut ret = self
        //     .route
        //     .windows(2)
        //     .map(|w| self.dist[w[0]][w[1]])
        //     .sum::<f64>();
        let ret = (0..N + 1)
            .collect_vec()
            .windows(2)
            .map(|w| self.dist[route[w[0] % N]][route[w[1] % N]])
            .sum::<f64>();
        // for i in 1..self.size {
        //     ret += self.dist[route[i - 1]][route[i]];
        // }
        // ret += self.dist[route[0]][route[self.size - 1]];
        ret
    }
    #[inline]
    fn evaluate_score(&mut self) {
        // let best_score = self.get_score(&self.best_route);
        let current_score = self.get_score(&self.route);
        if current_score < self.best_score {
            self.best_route = self.route.clone();
            self.best_score = current_score;
        }
    }
    #[inline]
    fn try_2opt(&self, a: usize, b: usize) -> bool {
        let N = self.size;
        let va0 = self.route[a % N];
        let va1 = self.route[(a + 1) % N];
        let vb0 = self.route[b % N];
        let vb1 = self.route[(b + 1) % N];

        let old = self.dist[va0][va1] + self.dist[vb0][vb1];
        let new = self.dist[va0][vb0] + self.dist[va1][vb1];
        // let (a0, a1) = (self.route[a - 1], self.route[a]);
        // let (b0, b1) = (self.route[b - 1], self.route[b]);
        // let old = self.dist[a0][a1] + self.dist[b0][b1];
        // let new = self.dist[a0][b0] + self.dist[a1][b1];
        new - old < 1e-5
    }
    #[inline]
    fn apply_2opt(&mut self, mut a: usize, mut b: usize) {
        let N = self.size;
        if (a + 1) % N > b % N {
            std::mem::swap(&mut a, &mut b);
        }
        self.route[(a + 1) % N..=b % N].reverse();
        for i in (a + 1) % N..=b % N {
            self.idx[self.route[i]] = i;
        }
        // if a > b {
        //     std::mem::swap(&mut a, &mut b);
        // }
        // self.route[a..b].reverse();
        // for i in a..b {
        //     self.idx[self.route[i]] = i;
        // }
    }
    fn ans(&self) {
        let mut ans: VecDeque<_> = self.best_route.iter().cloned().collect();
        while ans[0] != 0 {
            ans.rotate_right(1);
        }
        ans.push_back(0);
        // println!("{}", self.best_route.iter().map(|x| x + 1).join(" "));
        println!("{}", ans.iter().map(|x| x + 1).join(" "));
    }
}

#[inline]
fn local_search(state: &mut State, i0: &mut usize, improved: &mut bool) {
    let N = state.size;
    *improved = false;
    for i in *i0..*i0 + N {
        for j in i + 2..i + N - 1 {
            if state.try_2opt(i, j) {
                state.apply_2opt(i, j);
                *improved = true;
                *i0 = (i + 1) % N;
                break;
            }
        }
        if *improved {
            break;
        }
    }
}

fn solve1() {
    let mut state = State::new();
    let mut improved = true;
    let mut i0 = 0;
    while improved && get_time() < 0.98 {
        for _ in 0..8 {
            local_search(&mut state, &mut i0, &mut improved);
        }
        state.evaluate_score();
    }
    state.ans();
}

#[inline]
fn local_search2(state: &mut State) {
    let N = state.size;
    for a in 0..N {
        // for a in 1..=N {
        let va0 = state.route[a];
        let va1 = state.route[(a + 1) % N];
        // let va0 = state.route[a - 1];
        // let va1 = state.route[a];
        let current_dist = state.dist[va0][va1];

        for j in 0..N - 1 {
            let (d, nvb) = state.edge[va0][j];
            if current_dist <= d {
                break;
            }
            let b = state.idx[nvb];
            if (a as isize - b as isize).abs() > 1 && state.try_2opt(a, b) {
                state.apply_2opt(a, b);
                break;
            }
        }
    }
}

fn solve2() {
    let mut state = State::new();
    let mut iter = 0_usize;
    while get_time() < 0.98 {
        iter += 1;
        for _ in 0..8 {
            local_search2(&mut state);
        }
        state.evaluate_score();
    }
    eprintln!("iter: {}", iter);
    eprintln!("score: {}", state.get_score(&state.best_route));
    state.ans();
}

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        // get_time();`
        solve2();
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
