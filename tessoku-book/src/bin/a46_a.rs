#[allow(unused)]
mod rnd {
    static mut S: usize = 88172645463325252;

    #[inline]
    pub fn next() -> usize {
        unsafe {
            S ^= S << 7;
            S ^= S >> 9;
            S
        }
    }

    #[inline]
    pub fn range(a: usize, b: usize) -> usize {
        next() % (b - a) + a
    }
}

#[inline]
fn get_time_secs() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

struct Timer(f64);
impl Timer {
    #[inline]
    fn new() -> Timer {
        Timer(get_time_secs())
    }

    #[inline]
    fn get_time(&self) -> f64 {
        get_time_secs() - self.0
    }
}

use proconio::input;
use std::mem;

const N: usize = 150;

#[inline]
fn get_dist((x0, y0): (usize, usize), (x1, y1): (usize, usize)) -> f64 {
    let dx = x0 as i64 - x1 as i64;
    let dy = y0 as i64 - y1 as i64;

    ((dx * dx + dy * dy) as f64).sqrt()
}

struct State {
    // edge: [[(f64, usize); N - 1]; N],
    // dist: [[f64; N]; N],
    // route: [usize; N + 1],
    // pos: [usize; N],
    edge: Vec<Vec<(f64, usize)>>,
    dist: Vec<Vec<f64>>,
    route: Vec<usize>,
    pos: Vec<usize>,
}
impl State {
    #[inline]
    fn input() -> State {
        input! {
            n:usize,
            pos:[(usize,usize);n]
        }
        assert_eq!(n, N);

        let mut dist = vec![vec![0.; N]; N];
        let mut edge = vec![vec![(0., !0); N - 1]; N];

        for i in 0..N {
            let mut it = 0..;
            for j in 0..N {
                let d = get_dist(pos[i], pos[j]);
                dist[i][j] = d;
                if i != j {
                    edge[i][it.next().unwrap()] = (d, j);
                }
            }
            edge[i].sort_unstable_by(|&a, b| a.partial_cmp(b).unwrap());
        }

        let mut route = vec![!0; N + 1];
        for i in 0..N {
            route[i] = i;
        }
        route[N] = 0;

        let mut pos = vec![!0; N];
        for i in 1..N + 1 {
            pos[route[i]] = i;
        }

        State {
            edge,
            dist,
            route,
            pos,
        }
    }

    #[inline]
    fn score(&self) -> f64 {
        self.route
            .windows(2)
            .map(|w| self.dist[w[0]][w[1]])
            .sum::<f64>()
    }

    #[inline]
    fn try_2opt(&self, a: usize, b: usize) -> f64 {
        let (a0, a1) = (self.route[a - 1], self.route[a]);
        let (b0, b1) = (self.route[b - 1], self.route[b]);

        let old = self.dist[a0][a1] + self.dist[b0][b1];
        let new = self.dist[a0][b0] + self.dist[a1][b1];

        new - old
    }

    #[inline]
    fn apply_2opt(&mut self, mut a: usize, mut b: usize) {
        if a > b {
            mem::swap(&mut a, &mut b);
        }
        self.route[a..b].reverse();
        for i in a..b {
            self.pos[self.route[i]] = i;
        }
    }

    #[inline]
    fn apply_db(&mut self, a: usize, b: usize, c: usize, d: usize) {
        self.route[b..d].rotate_right(d - c);
        self.route[a..d].rotate_right(d - b);

        for i in a..d {
            self.pos[self.route[i]] = i;
        }
    }
}

#[inline]
fn isok(a: usize, b: usize) -> bool {
    (a as isize - b as isize).abs() > 1
}

fn local_search(state: &mut State) {
    for a in 1..=N {
        let v = state.route[a];
        let cur = state.dist[state.route[a - 1]][state.route[a]];

        for j in 0..N - 1 {
            let (d, nv) = state.edge[v][j];
            if cur <= d {
                break;
            }

            let b = state.pos[nv];
            if isok(a, b) {
                let diff = state.try_2opt(a, b);
                if diff < 1e-5 {
                    state.apply_2opt(a, b);
                    break;
                }
            }
        }
    }
}

fn kick(state: &mut State) {
    if rnd::next() & 1 == 0 {
        loop {
            let mut x = [!0; 4];
            for x in &mut x {
                *x = rnd::range(1, state.route.len());
            }
            x.sort_unstable();

            if x.windows(2).all(|w| isok(w[0], w[1])) {
                state.apply_db(x[0], x[1], x[2], x[3]);
            } else {
                continue;
            }
            break;
        }
    } else {
        for _ in 0..10 {
            let a = rnd::range(1, state.route.len());
            let mut b;
            while {
                b = rnd::range(1, state.route.len());

                !isok(a, b)
            } {}

            state.apply_2opt(a, b);
        }
    }
}

// fn solve(mut state: State, time: Timer) -> [usize; N + 1] {
fn solve(mut state: State, time: Timer) -> Vec<usize> {
    let mut best = std::f64::INFINITY;
    let mut best_state = vec![!0; N + 1];
    let mut iter = 0;

    let mut ty = 0;

    while time.get_time() <= 0.98 {
        iter += 1;
        for _ in 0..8 {
            local_search(&mut state);
        }
        let new = state.score();
        if new < best {
            best = new;
            best_state = state.route.clone();
        }
        // else {
        //     ty += 1;
        //     if ty >= 10 {
        //         ty = 0;
        //         state.route = best_state.clone();
        //         for i in 1..=N {
        //             state.pos[state.route[i]] = i;
        //         }
        //     }
        // }

        // kick(&mut state);
    }

    eprintln!("iter = {}", iter);
    eprintln!("score = {}", best);

    best_state
}

#[derive(Default)]
struct Solver {}
impl Solver {
    fn solve(&mut self) {
        let state = State::input();
        let time = Timer::new();

        let ans = solve(state, time);

        for i in 0..N + 1 {
            print!("{} ", ans[i] + 1);
        }
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
