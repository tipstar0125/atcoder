use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: u32
    }

    let mut vec: Vec<u32> = Vec::new();
    for _ in 0..10 {
        let m = N % 2;
        vec.push(m);
        N /= 2;
    }
    for v in vec.iter().rev() {
        print!("{:?}", v);
    }
}
