use std::process;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(usize, usize); m],
    }

    let mut g = vec![vec![]; n];
    for (a, b) in &ab {
        g[*a - 1].push(b - 1);
        g[*b - 1].push(a - 1);
    }

    let mut seen = vec![false; n];
    dfs(&g, &mut seen, 0);

    for i in 0..n {
        if seen[i] == false {
            println!("The graph is not connected.");
            process::exit(0)
        }
    }
    println!("The graph is connected.")
}

fn dfs(g: &Vec<Vec<usize>>, seen: &mut Vec<bool>, v: usize) {
    seen[v] = true;
    for i in 0..g[v].len() {
        let pos = g[v][i];
        if seen[pos] == false {
            dfs(&g, seen, pos);
        }
    }
    return;
}
