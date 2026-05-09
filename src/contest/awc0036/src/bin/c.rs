use itertools::Itertools;
use num_traits::pow;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashSet;
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut a = Vec::from(a);
    let mut lower = a.iter().copied().max().unwrap();
    let mut upper = a.iter().copied().sum::<usize>();

    while lower < upper {
        let mut mid = (lower + upper) / 2;
        if feasible(&a, &mid, k) {
            upper = mid;
        } else {
            lower = mid + 1;
        }
    }

    println!("{}", lower);
}

fn feasible(a: &Vec<usize>, x: &usize, k: usize) -> bool {
    let mut count = 1;
    let mut tol = 0;

    for v in a {
        if *v > *x {
            return false;
        }
        tol += *v;
        if tol > *x {
            count += 1;
            tol = *v;
            if count > k + 1 {
                return false;
            }
        }
    }
    true
}
