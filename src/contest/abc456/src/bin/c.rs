use im_rc::HashSet;
use itertools::Itertools;
use proconio::{input, marker::Chars};

const NUM: usize = 998244353;

fn main() {
    input! {
        s: Chars,
    }
    let mut ans = 0;
    let mut right = 0;
    for left in 0..s.len() {
        while right < s.len() - 1 && s[right] != s[right + 1] {
            right += 1;
        }

        ans += (right - left + 1);
        if left == right {
            right += 1;
        }
    }
    println!("{}", ans % NUM);
}
