use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut graph = vec![vec![]; n + 1];
    let mut seen = vec![false; n + 1];
    seen[0] = true;
    for i in 1..n + 1 {
        graph[i].push(0);
    }
    for (a, b) in ab {
        graph[a].push(b);
    }
    let mut count = 0;
    dfs(&graph, &mut seen, 1, &mut count);
    println!("{}", count);
}

fn dfs(g: &Vec<Vec<usize>>, seen: &mut Vec<bool>, now: usize, count: &mut usize) {
    seen[now] = true;
    *count += 1;
    for i in 0..g[now].len() {
        let next = g[now][i];
        if seen[next] {
            continue;
        }
        dfs(g, seen, next, count);
    }
}
