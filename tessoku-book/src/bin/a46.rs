#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use rand::Rng;
use std::time::Instant;

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
#[macro_export]
macro_rules! abs {
    ($x: expr) => {
        if $x < 0_isize {
            $x * (-1)
        } else {
            $x
        }
    };
}
#[macro_export]
macro_rules! absf {
    ($x: expr) => {
        if $x < 0.0 {
            $x * (-1.0)
        } else {
            $x
        }
    };
}

#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<isize>,
    size: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![-1; n + 1],
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

    fn unite(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return;
        }
        let size_x = -self.parent[root_x];
        let size_y = -self.parent[root_y];
        if size_x >= size_y {
            self.parent[root_x] -= size_y;
            self.parent[root_y] = root_x as isize;
        } else {
            self.parent[root_y] -= size_x;
            self.parent[root_x] = root_y as isize;
        }
        self.size -= 1;
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
}

#[derive(Debug, Clone)]
struct TimeKeeper {
    start_time_: Instant,
    time_threshold_: usize,
}

impl TimeKeeper {
    fn new(time_threshold_: usize) -> Self {
        TimeKeeper {
            start_time_: Instant::now(),
            time_threshold_,
        }
    }
    fn isTimeOver(&self) -> bool {
        self.start_time_.elapsed().as_micros() >= (self.time_threshold_ * 1e3 as usize) as u128
    }
}

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize,
            XY: [(f64, f64); N]
        }
        // play_greedy(N, XY);
        // play_hill_climbing(N, XY);
        // play_hill_climbing_with_time_keeper(N, XY);
        // play_annealing(N, XY);
        play_annealing_with_time_keeper(N, XY);
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

fn get_distance(x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    ((x0 - x1).powf(2.0) + (y0 - y1).powf(2.0)).sqrt()
}

fn get_score(ans: &Vec<usize>, XY: &[(f64, f64)]) -> f64 {
    let mut score = 0.0;
    for i in 0..ans.len() - 1 {
        let (x0, y0) = XY[ans[i]];
        let (x1, y1) = XY[ans[i + 1]];
        score += get_distance(x0, y0, x1, y1);
    }
    score
}

fn play_greedy(N: usize, XY: Vec<(f64, f64)>) {
    let mut visited = vec![false; N];
    let mut current = 0;
    let INF = (1_usize << 60) as f64;

    for _ in 0..N {
        visited[current] = true;
        let mut min = INF;
        let mut min_index = 0;
        for j in 0..N {
            if !visited[j] {
                let (x0, y0) = XY[current];
                let (x1, y1) = XY[j];
                let dist = get_distance(x0, y0, x1, y1);
                if dist < min {
                    min = dist;
                    min_index = j;
                }
            }
        }
        println!("{}", current + 1);
        current = min_index;
    }
    println!("1");
}

fn play_hill_climbing(N: usize, XY: Vec<(f64, f64)>) {
    let mut ans = vec![];
    for i in 0..N {
        ans.push(i);
    }
    ans.push(0);
    let mut current_score = get_score(&ans, &XY);
    let mut rng = rand::thread_rng();

    let MAX = 2e5 as usize;
    for _ in 1..=MAX {
        let mut L = rng.gen_range(1, N);
        let mut R = rng.gen_range(1, N);
        if L > R {
            std::mem::swap(&mut L, &mut R);
        } else if L == R {
            continue;
        }
        ans[L..=R].reverse();
        let new_score = get_score(&ans, &XY);
        if current_score >= new_score {
            current_score = new_score;
        } else {
            ans[L..=R].reverse();
        }
    }
    println!("{}", ans.iter().map(|x| x + 1).join("\n"));
}

fn play_hill_climbing_with_time_keeper(N: usize, XY: Vec<(f64, f64)>) {
    let time_keeper = TimeKeeper::new(990);
    let mut ans = vec![];
    for i in 0..N {
        ans.push(i);
    }
    ans.push(0);
    let mut current_score = get_score(&ans, &XY);
    let mut rng = rand::thread_rng();

    loop {
        let mut L = rng.gen_range(1, N);
        let mut R = rng.gen_range(1, N);
        if L > R {
            std::mem::swap(&mut L, &mut R);
        } else if L == R {
            continue;
        }
        ans[L..=R].reverse();
        let new_score = get_score(&ans, &XY);
        if current_score >= new_score {
            current_score = new_score;
        } else {
            ans[L..=R].reverse();
        }
        if time_keeper.isTimeOver() {
            break;
        }
    }
    println!("{}", ans.iter().map(|x| x + 1).join("\n"));
}

fn play_annealing(N: usize, XY: Vec<(f64, f64)>) {
    let mut ans = vec![];
    for i in 0..N {
        ans.push(i);
    }
    ans.push(0);
    let mut current_score = get_score(&ans, &XY);
    let mut rng = rand::thread_rng();

    let MAX = 1e6 as usize;
    for t in 1..=MAX {
        let mut L = rng.gen_range(1, N);
        let mut R = rng.gen_range(1, N);
        if L > R {
            std::mem::swap(&mut L, &mut R);
        } else if L == R {
            continue;
        }
        ans[L..=R].reverse();
        let new_score = get_score(&ans, &XY);

        let T = 30.0 - 28.0 * (t as f64) / (MAX as f64);
        let probability = ((current_score - new_score) / T).min(0.0).exp();
        if rng.gen::<f64>() < probability {
            current_score = new_score;
        } else {
            ans[L..=R].reverse();
        }
    }
    println!("{}", ans.iter().map(|x| x + 1).join("\n"));
}

fn play_annealing_with_time_keeper(N: usize, XY: Vec<(f64, f64)>) {
    let time_keeper = TimeKeeper::new(990);
    let mut ans = vec![];
    for i in 0..N {
        ans.push(i);
    }
    ans.push(0);
    let mut current_score = get_score(&ans, &XY);
    let mut rng = rand::thread_rng();

    let MAX = 1e6 as usize;
    for t in 1.. {
        let mut L = rng.gen_range(1, N);
        let mut R = rng.gen_range(1, N);
        if L > R {
            std::mem::swap(&mut L, &mut R);
        } else if L == R {
            continue;
        }
        ans[L..=R].reverse();
        let new_score = get_score(&ans, &XY);

        let T = (30.0 - 28.0 * (t as f64) / (MAX as f64)).max(2.0);
        let probability = ((current_score - new_score) / T).min(0.0).exp();
        if rng.gen::<f64>() < probability {
            current_score = new_score;
        } else {
            ans[L..=R].reverse();
        }
        if time_keeper.isTimeOver() {
            break;
        }
    }
    println!("{}", ans.iter().map(|x| x + 1).join("\n"));
}
