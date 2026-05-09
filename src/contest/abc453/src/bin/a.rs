use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        s: Chars,
    }
    let mut ans: Vec<char> = Vec::new();
    if s[0] != 'o' {
        println!("");
    } else {
        let mut is_ok = true;

        for i in 0..s.len() {
            if !is_ok {
                ans.push(s[i]);
            }
            if is_ok && s[i] != 'o' {
                is_ok = false;
                ans.push(s[i]);
            }
        }
        println!("{}", ans.iter().join(""));
    }
}
