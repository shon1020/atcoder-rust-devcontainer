use proconio::{input, marker::Usize1};
use std::collections::HashMap;
fn main() {
    input! {
        n: usize,
        m: usize,
        r: [usize; n],
        mut uv: [(Usize1, Usize1); m],
    }

    let mut cad_vec = Vec::new();

    for (idx, &v) in r.iter().enumerate() {
        cad_vec.push((idx, v));
    }

    cad_vec.sort_by(|x, y| y.1.cmp(&x.1));

    uv.sort_by(|x, y| x.1.cmp(&y.1));
    let mut map = HashMap::new();
    for (u, v) in uv {
        *map.entry(u).or_insert(vec![]).push(v);
    }

    let mut is_ok_vec = vec![false; n];
}
