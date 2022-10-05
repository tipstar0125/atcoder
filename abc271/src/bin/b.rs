use im_rc::HashMap;
use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize
    }
    
    let mut a = vec![];
    for _ in 0..N {
        input! {
            l_i: usize,
            a_i: [usize; l_i]
        }
        a.push(a_i)
    }
    input! {
        st: [(Usize1, Usize1); Q]
    }
    
    for &(s, t) in &st {
        println!("{:?}", a[s][t]);
    }
}
