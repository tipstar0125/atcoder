use proconio::{fastout, input};
use std::cmp::Ordering;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        Q: usize,
        LR:[(usize, usize); Q]
    }

    let mut atari = vec![0];
    let mut hazre = vec![0];

    for i in 0..N {
        if A[i] == 0 {
            atari.push(atari[i]);
            hazre.push(hazre[i] + 1);
        } else {
            atari.push(atari[i] + 1);
            hazre.push(hazre[i]);
        }
    }

    for &(L, R) in &LR {
        let num_atari = atari[R] - atari[L - 1];
        let num_hazre = hazre[R] - hazre[L - 1];

        match num_atari.cmp(&num_hazre) {
            Ordering::Greater => println!("win"),
            Ordering::Less => println!("lose"),
            Ordering::Equal => println!("draw"),
        }
    }
}
