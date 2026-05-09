use std::collections::VecDeque;

use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
fn main() {
    input! {
        n: usize,
        q: usize,
        cp: [(Usize1, Usize1); q],
    }
    let mut pointer = (0..n).collect_vec();
    let mut mountain = vec![vec![]; n];
    for i in 0..n {
        mountain[i].push(i);
    }

    for (c, p) in cp {
        let idx = pointer[c];
        let mut vec = Vec::new();
        let mut num = pow(10, 6);
        while mountain[idx].len() > 0 && num != c {
            num = mountain[idx].pop().unwrap();
            pointer[num] = pointer[p];
            vec.push(num);
        }
        while vec.len() > 0 {
            let v = vec.pop().unwrap();
            mountain[pointer[p]].push(v);
        }
    }

    println!("{}", mountain.iter().map(|a| a.len()).join(" "));
}
