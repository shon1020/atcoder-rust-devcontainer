use itertools::Itertools;
use num_traits::pow;
use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::collections::HashSet;
// やり直し
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let n = s.len();
    let m = t.len();

    let mut idx = vec![vec![]; 26];

    for i in 0..n {
        idx[s[i] as usize - 'a' as usize].push(i);
    }

    for i in 0..26 {
        idx[i].push(n);
    }
    println!("{:?}", idx);
    let mut ans: i64 = 0;

    for i in 0..n {
        let mut rig: i64 = i as i64 - 1;
        for j in 0..m {
            rig = idx[t[j] as usize - 'a' as usize].partition_point(|&x| x > rig as usize) as i64;
            rig += 1;
            if rig as usize == n {
                break;
            }
        }
        ans += rig - i as i64;
    }
    println!("{}", ans);
}
