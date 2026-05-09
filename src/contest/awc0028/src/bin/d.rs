use proconio::input;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut heap = BinaryHeap::new();
    let mut vec = vec![HashSet::new(); n + 1];
    let mut vec_g = vec![vec![]; n + 1];

    for (a, b) in &ab {
        vec[*b].insert(a);
        vec_g[*a].push(b);
    }

    for i in 1..(n + 1) {
        if vec[i].is_empty() {
            heap.push(Reverse(i));
        }
    }

    let mut ans = vec![];
    while !heap.is_empty() {
        if let Some(Reverse(num)) = heap.pop() {
            for x in &vec_g[num] {
                vec[**x].remove(&num);
                if vec[**x].is_empty() {
                    heap.push(Reverse(**x));
                }
            }
            ans.push(num);
        }
    }
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
