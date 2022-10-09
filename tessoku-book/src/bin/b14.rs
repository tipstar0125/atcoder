use proconio::{fastout, input};
use superslice::Ext;

#[allow(non_snake_case)]
#[allow(clippy::needless_range_loop)]
#[fastout]

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N]
    }

    let half_index = N / 2;
    let B = &A[..half_index];
    let C = &A[half_index..];

    let P = bit_full_search_sum_list(B);
    let Q = bit_full_search_sum_list(C);

    let mut ans = "No";
    for p in &P {
        let target = K - p;
        let index = Q.lower_bound(&target);
        if index < Q.len() && p + Q[index] == K {
            ans = "Yes";
        }
    }
    println!("{}", ans);
}

#[allow(clippy::needless_range_loop)]
fn bit_full_search_sum_list(vec: &[usize]) -> Vec<usize> {
    let mut sum_vec: Vec<_> = vec![];
    for bit in 0..(1 << vec.len()) {
        let mut sum = 0;
        for i in 0..vec.len() {
            if bit >> i & 1 == 1 {
                sum += vec[i];
            }
        }
        sum_vec.push(sum);
    }
    sum_vec.sort();
    sum_vec
}
