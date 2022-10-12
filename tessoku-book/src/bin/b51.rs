use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: Chars
    }

    let mut stack = vec![];
    for (i, c) in S.iter().enumerate() {
        match c {
            '(' => stack.push(i + 1),
            ')' => {
                println!("{} {}", stack.pop().unwrap(), i + 1);
            }
            _ => unreachable!(),
        }
    }
}
