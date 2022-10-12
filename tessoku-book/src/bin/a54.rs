use std::collections::HashMap;

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
                name: String,
                score: usize
            }
            Query.push((n, name, score));
        } else {
            input! {
                name: String,
            }
            Query.push((n, name, 0));
        }
    }

    let mut map = HashMap::new();
    for q in &Query {
        match q {
            (1, name, score) => {
                map.insert(name, score);
            }
            (2, name, _) => println!("{:?}", map.get(name).unwrap()),
            (_, _, _) => unreachable!(),
        }
    }
}
