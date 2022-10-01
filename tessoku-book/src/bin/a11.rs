use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]

fn main() {
    input! {
        N: usize,
        X: usize,
        A: [usize; N]
    }

    let ans = binary_search(&X, &A).unwrap() + 1;
    println!("{:?}", ans);
}

fn binary_search(x: &usize, v: &Vec<usize>) -> Option<usize> {
    let mut left = 0;
    let mut right = v.len() - 1;

    while left <= right {
        let med = (left + right) / 2;
        if *x < v[med] {
            right = med - 1;
        }
        if *x == v[med] {
            return Some(med);
        }
        if *x >= v[med] {
            left = med + 1;
        }
    }
    None
}
