use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32
    }

    let mut exists = false;
    for i in a..=b {
        if (100 % i) == 0 {
            exists = true;
            break;
        }
    }
    if exists {
        println!("Yes");
    } else {
        println!("No");
    }
}
