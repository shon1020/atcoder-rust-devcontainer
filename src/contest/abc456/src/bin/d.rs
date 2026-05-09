use im_rc::HashSet;
use itertools::Itertools;
use proconio::{input, marker::Chars};

const NUM: usize = 998244353;

fn main() {
    input! {
        s: Chars,
    }
    let mut ans = 0;
    let mut right = 1;
    for left in 0..s.len() {
        while right < s.len() - 1 && s[right - 1] != s[right] {
            right += 1;
        }

        ans += (right + 1);
        if left == right {
            right += 1;
        }
    }
    println!("{}", ans % NUM);
}
