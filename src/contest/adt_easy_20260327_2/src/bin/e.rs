use std::process;

use itertools::all;
use pathfinding::grid;
use proconio::input;
use proconio::marker::Usize1;
fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut seen = vec![false; n];
    let mut count = 0;
    if (n - 1) != m {
        println!("No");
        process::exit(0);
    }
    for e in &graph {
        if e.len() > 2 {
            println!("No");
            process::exit(0);
        }
    }
    dfs(&graph, &mut seen, 0);
    yesno(seen.iter().all(|&x| x));
}

fn dfs(g: &Vec<Vec<usize>>, seen: &mut Vec<bool>, num: usize) {
    seen[num] = true;
    for i in 0..g[num].len() {
        let x = g[num][i];
        if seen[x] {
            continue;
        }
        dfs(g, seen, x);
    }
    return;
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
