use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut graph = vec![vec![]; n + 1];
    let mut seen = vec![false; n + 1];
    let mut path = vec![];
    let mut answer = Vec::new();

    for (a, b) in &ab {
        graph[*a].push(b);
        graph[*b].push(a);
    }
    path.push(1);
    dfs(&graph, &mut seen, &mut path, 1, &n, &mut answer);
    for i in 0..answer.len() {
        print!("{} ", answer[i]);
    }
}

fn dfs(
    g: &Vec<Vec<&usize>>,
    seen: &mut Vec<bool>,
    path: &mut Vec<usize>,
    v: usize,
    n: &usize,
    answer: &mut Vec<usize>,
) {
    seen[v] = true;
    if v == *n {
        *answer = path.clone();
        return;
    }
    for i in 0..g[v].len() {
        let pos = g[v][i];
        if seen[*pos] == false {
            path.push(*pos);
            dfs(g, seen, path, *pos, n, answer);
            path.pop();
        }
    }
    return;
}
