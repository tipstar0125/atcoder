use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]

fn main() {
    input! {
        N: usize,
        A: [usize; N],
        D: usize,
        LR: [(usize, usize); D]
    }

    for (L, R) in LR {
        let mut B = A.clone();
        B.drain(L - 1..R);
        B.sort();
        println!("{:?}", B.last().unwrap());
    }
}
