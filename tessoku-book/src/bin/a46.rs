#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]

use rand::rngs::StdRng;
use std::{
    collections::{BTreeMap, BTreeSet},
    time::Instant,
};

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use rand::Rng;

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
            (ms - STIME) * 0.85
        }
        #[cfg(not(feature = "local"))]
        {
            ms - STIME
        }
    }
}

#[derive(Debug, Clone)]
struct TimeKeeper {
    start_time: Instant,
    time_threshold: f64, // us
}

impl TimeKeeper {
    fn new(ms: usize) -> Self {
        TimeKeeper {
            start_time: Instant::now(),
            time_threshold: (ms * 1e3 as usize) as f64,
        }
    }
    #[inline]
    fn isTimeOver(&self) -> bool {
        let elapsed_time = self.start_time.elapsed().as_micros() as f64;
        #[cfg(feature = "local")]
        {
            elapsed_time * 0.85 >= self.time_threshold
        }
        #[cfg(not(feature = "local"))]
        {
            elapsed_time >= self.time_threshold
        }
    }
}

#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<isize>,
    size: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![-1; n],
            size: n,
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            return x;
        }
        let root = self.find(self.parent[x] as usize);
        self.parent[x] = root as isize;
        root
    }
    fn unite(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return None;
        }
        let size_x = -self.parent[root_x];
        let size_y = -self.parent[root_y];
        self.size -= 1;
        if size_x >= size_y {
            self.parent[root_x] -= size_y;
            self.parent[root_y] = root_x as isize;
            Some((root_x, root_y))
        } else {
            self.parent[root_y] -= size_x;
            self.parent[root_x] = root_y as isize;
            Some((root_y, root_x))
        }
    }
    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    fn is_root(&mut self, x: usize) -> bool {
        self.find(x) == x
    }
    fn get_union_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        -self.parent[root] as usize
    }
    fn get_size(&self) -> usize {
        self.size
    }
    fn roots(&self) -> Vec<usize> {
        (0..self.parent.len())
            .filter(|i| self.parent[*i] < 0)
            .collect::<Vec<usize>>()
    }
    fn members(&mut self, x: usize) -> Vec<usize> {
        let root = self.find(x);
        (0..self.parent.len())
            .filter(|i| self.find(*i) == root)
            .collect::<Vec<usize>>()
    }
    fn all_group_members(&mut self) -> BTreeMap<usize, Vec<usize>> {
        let mut groups_map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for x in 0..self.parent.len() {
            let r = self.find(x);
            groups_map.entry(r).or_default().push(x);
        }
        groups_map
    }
}

fn get_dist((x0, y0): (isize, isize), (x1, y1): (isize, isize)) -> f64 {
    let dx = x0 - x1;
    let dy = y0 - y1;
    ((dx * dx + dy * dy) as f64).sqrt()
}

#[derive(Debug)]
struct State {
    N: usize,
    edge: Vec<Vec<(f64, usize)>>,
    dist: Vec<Vec<f64>>,
    route: Vec<usize>,
    best_route: Vec<usize>,
    best_score: f64,
    pos: Vec<usize>,
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

        let route = (0..N).chain(vec![0]).collect_vec();
        let best_route = route.clone();
        let best_score = std::f64::INFINITY;
        let mut pos = vec![0; N];
        for i in 1..=N {
            pos[route[i]] = i;
        }

        State {
            N,
            edge,
            dist,
            route,
            best_route,
            best_score,
            pos,
        }
    }
    fn init(&mut self) {
        self.kruskal();
    }
    fn greedy(&mut self) {
        let mut route = vec![];
        let mut visited = vec![false; self.N];
        let start = 0;
        let mut now = start;
        route.push(start);
        visited[start] = true;

        for _ in 0..self.N - 1 {
            for i in 0..self.N - 1 {
                let (_, next) = self.edge[now][i];
                if !visited[next] {
                    visited[next] = true;
                    route.push(next);
                    now = next;
                    break;
                }
            }
        }
        route.push(0);

        for i in 1..=self.N {
            self.pos[route[i]] = i;
        }
        self.route = route;
        self.evaluate_score();
    }
    fn kruskal(&mut self) {
        let mut connection = vec![];
        for (i, v) in self.edge.iter().enumerate() {
            for (d, j) in v {
                connection.push((i, *j, d));
            }
        }
        connection.sort_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap());
        let mut uf = UnionFind::new(self.N);
        let mut relation = vec![vec![]; self.N];

        for &(i, j, _) in &connection {
            if !uf.is_same(i, j) && relation[i].len() < 2 && relation[j].len() < 2 {
                if (i == 0 && !relation[i].is_empty()) || (j == 0 && !relation[j].is_empty()) {
                    continue;
                }
                uf.unite(i, j);
                relation[i].push(j);
                relation[j].push(i);
            }
        }

        let start = 0;
        let mut visited = vec![false; self.N];
        let mut route = vec![];
        let mut now = start;
        visited[start] = true;
        route.push(start);

        while route.len() < self.N {
            let mut iter = relation[now].iter();
            let mut nex = *iter.next().unwrap();
            if visited[nex] {
                nex = *iter.next().unwrap();
            }
            route.push(nex);
            visited[nex] = true;
            now = nex;
        }
        route.push(0);

        for i in 1..=self.N {
            self.pos[route[i]] = i;
        }
        self.route = route;
        self.evaluate_score();
    }
    fn annealing(&mut self, rng: &mut StdRng) {
        let MAX = 1e5 as usize;
        let mut current_score = self.get_score();
        for t in 1..=MAX {
            let mut a = rng.gen_range(1, self.N + 1);
            let mut b = rng.gen_range(1, self.N + 1);
            if !self.legal_check(a, b) {
                continue;
            }
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            self.route[a..b].reverse();
            let new_score = self.get_score();

            let T = 30.0 - 28.0 * (t as f64) / (MAX as f64);
            let prob = ((current_score - new_score) / T).min(0.0).exp();
            if rng.gen::<f64>() < prob {
                current_score = new_score;
            } else {
                self.route[a..b].reverse();
            }
        }
        for i in 1..=self.N {
            self.pos[self.route[i]] = i;
        }
        self.evaluate_score();
    }
    #[inline]
    fn get_score(&self) -> f64 {
        self.route
            .windows(2)
            .map(|w| self.dist[w[0]][w[1]])
            .sum::<f64>()
    }
    #[inline]
    fn evaluate_score(&mut self) -> bool {
        let current_score = self.get_score();
        let updated = current_score < self.best_score;
        if updated {
            self.best_route = self.route.clone();
            self.best_score = current_score;
        }
        updated
    }
    #[inline]
    fn try_2opt(&self, a: usize, b: usize) -> bool {
        let (va0, va1) = (self.route[a - 1], self.route[a]);
        let (vb0, vb1) = (self.route[b - 1], self.route[b]);
        let old = self.dist[va0][va1] + self.dist[vb0][vb1];
        let new = self.dist[va0][vb0] + self.dist[va1][vb1];
        new - old < 1e-5
    }
    #[inline]
    fn apply_2opt(&mut self, mut a: usize, mut b: usize) {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        self.route[a..b].reverse();
        for i in a..b {
            self.pos[self.route[i]] = i;
        }
    }
    #[inline]
    fn apply_double_bridge(&mut self, a: usize, b: usize, c: usize, d: usize) {
        self.route[b..d].rotate_right(d - c);
        self.route[a..d].rotate_right(d - b);
        for i in a..d {
            self.pos[self.route[i]] = i;
        }
    }
    #[inline]
    fn legal_check(&self, a: usize, b: usize) -> bool {
        (a as isize - b as isize).abs() > 1
    }
    #[inline]
    fn kick(&mut self, rng: &mut StdRng) {
        if rng.gen::<bool>() {
            let mut x = [0; 4];
            while !x.windows(2).all(|w| self.legal_check(w[0], w[1])) {
                for xi in x.iter_mut() {
                    *xi = rng.gen_range(1, self.N);
                }
                x.sort();
            }
            self.apply_double_bridge(x[0], x[1], x[2], x[3]);
        } else {
            for _ in 0..10 {
                let a = rng.gen_range(1, self.N + 1);
                let mut b = rng.gen_range(1, self.N + 1);
                while !self.legal_check(a, b) {
                    b = rng.gen_range(1, self.N + 1);
                }
                self.apply_2opt(a, b);
            }
        }
    }
    fn ans(&self) {
        println!("{}", self.best_route.iter().map(|x| x + 1).join(" "));
    }
}

#[inline]
fn local_search(state: &mut State) {
    for a in 1..=state.N {
        let va0 = state.route[a - 1];
        let va1 = state.route[a];
        let current_dist = state.dist[va0][va1];

        for j in 0..state.N - 1 {
            let (d, vb1) = state.edge[va1][j];
            if current_dist <= d {
                break;
            }
            let b = state.pos[vb1];
            if state.legal_check(a, b) && state.try_2opt(a, b) {
                state.apply_2opt(a, b);
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
        #[allow(unused_mut)]
        let mut seed = rand::thread_rng().gen();
        #[cfg(feature = "seed")]
        {
            seed = 11216848234635351618;
        }
        eprintln!("seed: {}", seed);
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);

        let mut state = State::new();
        let time_keeper = TimeKeeper::new(980);

        state.init();
        state.annealing(&mut rng);
        let mut iter = 0_usize;
        let mut try_num = 0;

        while !time_keeper.isTimeOver() {
            iter += 1;
            for _ in 0..8 {
                local_search(&mut state);
            }
            if !state.evaluate_score() {
                try_num += 1;
                if try_num >= 10 {
                    try_num = 0;
                    state.route = state.best_route.clone();
                    for i in 1..=state.N {
                        state.pos[state.route[i]] = i;
                    }
                }
            }
            state.kick(&mut rng);
        }
        eprintln!("iter: {}", iter);
        eprintln!("score: {}", state.best_score);
        state.ans();
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
