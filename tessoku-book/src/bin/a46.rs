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

fn get_dist((x0, y0): (isize, isize), (x1, y1): (isize, isize)) -> NotNan<f64> {
    let dx = x0 - x1;
    let dy = y0 - y1;
    NotNan::new(((dx * dx + dy * dy) as f64).sqrt()).unwrap()
}

#[derive(Debug)]
struct State {
    size: usize,
    edge: Vec<Vec<(NotNan<f64>, usize)>>,
    dist: Vec<Vec<NotNan<f64>>>,
    route: Vec<usize>,
    // pos: Vec<usize>,
}

impl State {
    fn new() -> Self {
        input! {
            N: usize,
            XY: [(isize, isize); N]
        }
        let mut dist = vec![vec![NotNan::new(0.).unwrap(); N]; N];
        let mut edge = vec![vec![]; N];

        for i in 0..N {
            for j in 0..N {
                let d = get_dist(XY[i], XY[j]);
                dist[i][j] = d;
                if i != j {
                    edge[i].push((d, j));
                }
            }
            edge[i].sort();
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

        // for i in 0..N {
        //     route.push(i);
        // }
        // route.push(0);

        // let mut pos = vec![0; N];

        State {
            size: N,
            edge,
            dist,
            route,
            // pos,
        }
    }
    fn get_score(&self) -> NotNan<f64> {
        let mut ret = NotNan::new(0.).unwrap();
        for i in 1..self.size {
            ret += self.dist[self.route[i - 1]][self.route[i]];
        }
        ret += self.dist[self.route[0]][self.route[self.size - 1]];
        ret
    }
    fn try_2opt(&self, a: usize, b: usize) -> bool {
        let N = self.size;
        self.dist[self.route[a % N]][self.route[b % N]]
            + self.dist[self.route[(a + 1) % N]][self.route[(b + 1) % N]]
            < self.dist[self.route[a % N]][self.route[(a + 1) % N]]
                + self.dist[self.route[b % N]][self.route[(b + 1) % N]]
                + NotNan::new(1e-5).unwrap()
    }
    fn apply_2opt(&mut self, a: usize, b: usize) {
        let N = self.size;
        self.route[(a + 1) % N..=b % N].reverse();
    }
}

fn local_search(state: &mut State) {
    let mut i0 = 0_usize;
    let mut improved = true;
    let N = state.size;
    while improved && get_time() < 0.9 {
        improved = false;
        for i in i0..i0 + N {
            for j in i + 2..i + N - 1 {
                if state.try_2opt(i, j) {
                    if (i + 1) % N < j % N {
                        state.apply_2opt(i, j);
                    } else {
                        state.apply_2opt(j, i);
                    }
                    improved = true;
                    i0 = (i + 1) % N;
                    break;
                }
            }
            if improved {
                break;
            }
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
        local_search(&mut state);
        eprintln!("{}", state.get_score());

        let mut ans: VecDeque<usize> = state.route.iter().cloned().collect();
        while ans[0] != 0 {
            let tail = ans.pop_back().unwrap();
            ans.push_front(tail);
        }
        ans.push_back(0);
        println!("{}", ans.iter().map(|x| x + 1).join(" "));
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
