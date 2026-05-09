use indexing::algorithms::lower_bound;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars,
    }
    let prefix_a = prefix(&s, 'a');
    let prefix_b = prefix(&s, 'b');

    let mut ans: i64 = 0;
    for i in 0..n {
        let a_pos = lower_bound(&prefix_a, &(prefix_a[i] + a));
        let b_pos = lower_bound(&prefix_b, &(prefix_b[i] + b));
        ans += max(b_pos as i64 - a_pos as i64, 0);
    }
    println!("{}", ans);
}

fn prefix(s: &Vec<char>, c: char) -> Vec<usize> {
    let mut prefix: Vec<usize> = vec![0; s.len() + 1];
    for i in 1..s.len() + 1 {
        if s[i - 1] == c {
            prefix[i] = prefix[i - 1] + 1;
        } else {
            prefix[i] = prefix[i - 1];
        }
    }
    return prefix;
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
