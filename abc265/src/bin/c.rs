#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        G: [Chars; H]
    }
    let mut i = 0;
    let mut j = 0;
    let mut visited = vec![vec![false; W]; H];
    let mut is_ng = false;
    loop {
        visited[i][j] = true;
        match G[i][j] {
            'U' => {
                if i != 0 {
                    i -= 1;
                } else {
                    break;
                }
            }
            'D' => {
                if i != H - 1 {
                    i += 1;
                } else {
                    break;
                }
            }
            'L' => {
                if j != 0 {
                    j -= 1;
                } else {
                    break;
                }
            }
            'R' => {
                if j != W - 1 {
                    j += 1;
                } else {
                    break;
                }
            }
            _ => unreachable!(),
        }
        if visited[i][j] {
            is_ng = true;
            break;
        }
    }

    if is_ng {
        println!("-1");
    } else {
        println!("{} {}", i + 1, j + 1);
    }
}
