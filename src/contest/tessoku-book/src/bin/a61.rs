use std::collections::HashSet;

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut graph = vec![vec![]; n + 1];
    for (a, b) in &ab {
        graph[*a].push(b);
        graph[*b].push(a);
    }

    for i in 1..(n + 1) {
        print!("{}: {{", i);
        for j in 0..graph[i].len() {
            if j >= 1 {
                print!(", ");
            }
            print!("{}", graph[i][j]);
        }
        println!("}}");
    }
}
