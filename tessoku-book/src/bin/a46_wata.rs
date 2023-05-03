#![allow(non_snake_case)]

use proconio::*;
use rand::prelude::*;

fn main() {
    get_time();
    let input = read_input();
    let out = solve(&input);
    eprintln!("Time = {:.3}", get_time());
    write_output(&out);
}

fn solve(input: &Input) -> Output {
    let mut dist = mat![0.0; input.N; input.N];
    for i in 0..input.N {
        for j in 0..input.N {
            let dx = input.ps[i].0 - input.ps[j].0;
            let dy = input.ps[i].1 - input.ps[j].1;
            dist[i][j] = (dx * dx + dy * dy).sqrt();
        }
    }
    let qs = (0..input.N).chain(vec![0]).collect();
    tsp(&dist, &qs, 0.95)
}

type Output = Vec<usize>;

#[derive(Clone, Debug)]
pub struct Input {
    pub N: usize,
    pub ps: Vec<(f64, f64)>,
}

pub fn read_input() -> Input {
    input! {
        N: usize,
        ps: [(f64, f64); N],
    }
    Input { N, ps }
}

pub fn write_output(out: &Output) {
    assert_eq!(out[0], 0);
    for i in out {
        println!("{}", i + 1);
    }
}

// ここからライブラリ

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { vec![$($e),*] };
	($($e:expr,)*) => { vec![$($e),*] };
	($e:expr; $d:expr) => { vec![$e; $d] };
	($e:expr; $d:expr $(; $ds:expr)+) => { vec![mat![$e $(; $ds)*]; $d] };
}

pub fn get_time() -> f64 {
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

pub fn compute_cost(g: &Vec<Vec<f64>>, ps: &Vec<usize>) -> f64 {
    let mut tmp = 0.0;
    for i in 0..ps.len() - 1 {
        tmp += g[ps[i]][ps[i + 1]];
    }
    tmp
}

// mv: (i, dir)
pub fn apply_move(tour: &mut Vec<usize>, idx: &mut Vec<usize>, mv: &[(usize, usize)]) {
    let k = mv.len();
    let mut ids: Vec<_> = (0..k).collect();
    ids.sort_by_key(|&i| mv[i].0);
    let mut order = vec![0; k];
    for i in 0..k {
        order[ids[i]] = i;
    }
    let mut tour2 = Vec::with_capacity(mv[ids[k - 1]].0 - mv[ids[0]].0);
    let mut i = ids[0];
    let mut dir = 0;
    loop {
        let (j, rev) = if dir == mv[i].1 {
            ((i + 1) % k, 0)
        } else {
            ((i + k - 1) % k, 1)
        };
        if mv[j].1 == rev {
            if order[j] == k - 1 {
                break;
            } else {
                i = ids[order[j] + 1];
                dir = 0;
                tour2.extend_from_slice(&tour[mv[j].0 + 1..mv[i].0 + 1]);
            }
        } else {
            i = ids[order[j] - 1];
            dir = 1;
            tour2.extend(tour[mv[i].0 + 1..mv[j].0 + 1].iter().rev().cloned());
        }
    }
    assert_eq!(tour2.len(), mv[ids[k - 1]].0 - mv[ids[0]].0);
    tour[mv[ids[0]].0 + 1..mv[ids[k - 1]].0 + 1].copy_from_slice(&tour2);
    for i in mv[ids[0]].0 + 1..mv[ids[k - 1]].0 + 1 {
        idx[tour[i]] = i;
    }
}

pub const FEASIBLE3: [bool; 64] = [
    false, false, false, true, false, true, true, true, true, true, true, false, true, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    true, false, true, true, true, true, true, true, false, true, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, true, false, true, true,
    true, true, true, true, false, true, false, false, false,
];

pub const EPS: f64 = 1e-5;

pub fn tsp(g: &Vec<Vec<f64>>, qs: &Vec<usize>, until: f64) -> Vec<usize> {
    let n = g.len();
    let mut f = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if i != j {
                f[i].push((g[i][j], j));
            }
        }
        f[i].sort_by(|&(a, _), &(b, _)| a.partial_cmp(&b).unwrap());
    }
    let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(4372982);
    let mut ps = qs.clone();
    let mut idx = vec![!0; n];
    let (mut min, mut min_ps) = (compute_cost(&g, &qs), ps.clone());
    while get_time() < until {
        let mut cost = compute_cost(&g, &ps);
        for p in 0..n {
            idx[ps[p]] = p;
        }
        loop {
            let mut ok = false;
            for i in 0..n {
                for di in 0..2 {
                    'loop_ij: for &(ij, vj) in &f[ps[i + di]] {
                        if g[ps[i]][ps[i + 1]] - ij <= EPS {
                            break;
                        }
                        for dj in 0..2 {
                            let j = if idx[vj] == 0 && dj == 0 {
                                n - 1
                            } else {
                                idx[vj] - 1 + dj
                            };
                            let gain = g[ps[i]][ps[i + 1]] - ij + g[ps[j]][ps[j + 1]];
                            // 2-opt
                            if di != dj && gain - g[ps[j + dj]][ps[i + 1 - di]] > EPS {
                                cost -= gain - g[ps[j + dj]][ps[i + 1 - di]];
                                apply_move(&mut ps, &mut idx, &[(i, di), (j, dj)]);
                                ok = true;
                                break 'loop_ij;
                            }
                            // 3-opt
                            for &(jk, vk) in &f[ps[j + dj]] {
                                if gain - jk <= EPS {
                                    break;
                                }
                                for dk in 0..2 {
                                    let k = if idx[vk] == 0 && dk == 0 {
                                        n - 1
                                    } else {
                                        idx[vk] - 1 + dk
                                    };
                                    if i == k || j == k {
                                        continue;
                                    }
                                    let gain = gain - jk + g[ps[k]][ps[k + 1]];
                                    if gain - g[ps[k + dk]][ps[i + 1 - di]] > EPS {
                                        let mask = if i < j { 1 << 5 } else { 0 }
                                            | if i < k { 1 << 4 } else { 0 }
                                            | if j < k { 1 << 3 } else { 0 }
                                            | di << 2
                                            | dj << 1
                                            | dk;
                                        if FEASIBLE3[mask] {
                                            cost -= gain - g[ps[k + dk]][ps[i + 1 - di]];
                                            apply_move(
                                                &mut ps,
                                                &mut idx,
                                                &[(i, di), (j, dj), (k, dk)],
                                            );
                                            ok = true;
                                            break 'loop_ij;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if !ok {
                break;
            }
        }
        if min.setmin(cost) {
            min_ps = ps;
        }
        ps = min_ps.clone();
        if n <= 4 {
            break;
        }
        loop {
            if rng.gen_range(0, 2) == 0 {
                // double bridge
                let mut is: Vec<_> = (0..4).map(|_| rng.gen_range(0, n)).collect();
                is.sort();
                if is[0] == is[1] || is[1] == is[2] || is[2] == is[3] {
                    continue;
                }
                ps = ps[0..is[0] + 1]
                    .iter()
                    .chain(ps[is[2] + 1..is[3] + 1].iter())
                    .chain(ps[is[1] + 1..is[2] + 1].iter())
                    .chain(ps[is[0] + 1..is[1] + 1].iter())
                    .chain(ps[is[3] + 1..].iter())
                    .cloned()
                    .collect();
            } else {
                for _ in 0..6 {
                    loop {
                        let i = rng.gen_range(1, n);
                        let j = rng.gen_range(1, n);
                        if i < j && j - i < n - 2 {
                            ps = ps[0..i]
                                .iter()
                                .chain(ps[i..j + 1].iter().rev())
                                .chain(ps[j + 1..].iter())
                                .cloned()
                                .collect();
                            break;
                        }
                    }
                }
            }
            break;
        }
    }
    min_ps
}
