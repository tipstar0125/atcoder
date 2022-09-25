use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: u32
    }

    for x in (0..10).rev() {
        let wari = 1 << x;
        print!("{:?}", (N / wari) % 2);
    }
    println!();
    // let mut vec: Vec<u32> = Vec::new();
    // for _ in 0..10 {
    //     let m = N % 2;
    //     vec.push(m);
    //     N /= 2;
    // }
    // for v in vec.iter().rev() {
    //     print!("{:?}", v);
    // }
}
