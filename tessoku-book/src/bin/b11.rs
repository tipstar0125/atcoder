use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]

fn main() {
    input! {
        N: usize,
        A: [usize; N],
        Q: usize,
        X: [usize; Q]
    }

    for x in &X {
        let ans = lower_bound(x, A).unwrap();
        println!("{:?}", ans);
    }
}

fn lower_bound(&x: usize, v: Vec<usize>) -> Option<usize> {
    let mut left = 0;
    let mut right = v.len() - 1;

    while left <= right {
        let med = (left + right) / 2;
        if x < v[med] {
            right = med - 1;
        }
        if x == v[med] {
            return Some(med);
        }
        if x >= v[med] {
            left = med + 1;
        }
    }
    None
}
