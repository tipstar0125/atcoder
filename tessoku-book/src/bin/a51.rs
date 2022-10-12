use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        Q: usize,
    }

    let mut Query = vec![];
    for _ in 0..Q {
        input! {
            n: usize,
        }
        if n == 1 {
            input! {
               name: String
            }
            Query.push((n, name));
        } else {
            Query.push((n, "".to_string()));
        }
    }

    let mut stack = vec![];
    for q in &Query {
        match q {
            (1, v) => stack.push(v),
            (2, _) => println!("{}", stack.last().unwrap()),
            (3, _) => {
                stack.pop();
            }
            (_, _) => unreachable!(),
        }
    }
}
