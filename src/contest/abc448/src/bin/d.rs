use std::collections::HashSet;

use proconio::input;
//問題文読み間違いやり直し
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        uv: [(usize, usize); n-1]
    }

    let mut graph = vec![vec![]; n + 1];
    let mut seen = vec![false; n + 1];
    let mut cad: HashSet<usize> = HashSet::new();
    for (u, v) in &uv {
        graph[*u].push(v);
        graph[*v].push(u);
    }
    dfs(&graph, &mut seen, &a, &mut cad, 0, 1);

    for i in 1..(n + 1) {
        yesno(seen[i]);
    }
}

fn dfs(
    g: &Vec<Vec<&usize>>,
    seen: &mut Vec<bool>,
    a: &Vec<usize>,
    cad: &mut HashSet<usize>,
    pre_v: usize,
    v: usize,
) {
    let val = a[v - 1];
    if seen[v] == seen[pre_v] {
        seen[v] = true;
    } else if cad.contains(&val) {
        seen[v] = true;
    }
    cad.insert(val);
    for i in 0..g[v].len() {
        let pos = g[v][i];
        if pre_v == *pos {
            continue;
        }

        if cad.contains(&a[pos - 1]) {
            continue;
        }
        dfs(g, seen, a, cad, v, *pos);
    }
    cad.remove(&val);
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
