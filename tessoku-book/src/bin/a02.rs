use proconio::input;
fn main() {
    input! {
        n: u32,
        x: u32,
        a: [u32; n]
    }

    if a.iter().any(|e| *e == x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
