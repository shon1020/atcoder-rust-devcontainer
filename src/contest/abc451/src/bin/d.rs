use itertools::Itertools;
use num_traits::pow;
use proconio::input;
fn main() {
    input! {
        n: usize,
    }

    let mut beki_vec: Vec<i64> = vec![];
    for i in 0..30 {
        beki_vec.push(1 << i);
    }
    let mut shift = 0;
    for x in 0..30 {
        if pow(2, x) <= n {
            shift = x;
        } else {
            break;
        }
    }
    let mut ans_vec = vec![];
    for i in 0..shift + 1 {
        for comb in beki_vec.iter().combinations_with_replacement(i) {
            for x in comb {
                ans_vec.push(x);
            }
        }
    }
    println!("{}", ans_vec[n - 1]);
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
