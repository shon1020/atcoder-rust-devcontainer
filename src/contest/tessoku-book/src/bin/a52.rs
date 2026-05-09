use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
    }

    let mut queue = VecDeque::new();

    for _ in 0..q {
        input! {
            x: usize,
        }
        match x {
            1 => {
                input! {
                    name: String,
                }
                queue.push_front(name);
            }

            2 => {
                println!("{}", queue.back().unwrap());
            }

            3 => {
                queue.pop_back();
            }

            _ => (),
        }
    }
}
