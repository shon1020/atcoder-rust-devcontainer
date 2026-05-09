use itertools::Itertools;
use pathfinding::matrix::directions::W;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        r: [usize; n],
    }

    let mut ans_vec = Vec::new();
    let mut cad_vec = Vec::new();
    dfs(&r, &mut ans_vec, &mut cad_vec, n, k, 0);
    for ans in ans_vec {
        println!("{}", ans.iter().join(" "));
    }
}

fn dfs(
    r: &Vec<usize>,
    ans_vec: &mut Vec<Vec<usize>>,
    cad_vec: &mut Vec<usize>,
    n: usize,
    k: usize,
    num: usize,
) {
    if cad_vec.len() == n {
        if cad_vec.iter().sum::<usize>() % k == 0 {
            ans_vec.push(cad_vec.clone());
            return;
        } else {
            return;
        }
    }

    for i in 1..r[num] + 1 {
        cad_vec.push(i);
        let num = num + 1;
        dfs(r, ans_vec, cad_vec, n, k, num);
        cad_vec.pop();
    }
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
