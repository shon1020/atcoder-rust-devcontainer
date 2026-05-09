use itertools::Itertools;
use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        q: usize,
    }
    let mut queue = BinaryHeap::new();
    for _ in 0..q {
        input! {
            n: usize,
        }
        match n {
            1 => {
                input! {
                    h: i64,
                }
                queue.push((-h, h));
                println!("{}", queue.len());
            }

            2 => {
                input! {
                    h: i64,
                }

                while queue.len() >= 1 && queue.peek().unwrap().1 <= h {
                    queue.pop();
                }
                println!("{}", queue.len());
            }
            _ => (),
        };
    }
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
