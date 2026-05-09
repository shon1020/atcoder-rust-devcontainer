use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;
fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: Chars,
    }

    let mut q = VecDeque::new();
    q.push_front(x - 1);
    a[x - 1] = '@';

    while !q.is_empty() {
        let pos: usize = q.pop_back().unwrap();
        if pos > 0 && a[pos - 1] == '.' {
            a[pos - 1] = '@';
            q.push_front(&pos - 1);
        }
        if pos < (n - 1) && a[pos + 1] == '.' {
            a[pos + 1] = '@';
            q.push_front(pos + 1);
        }
    }

    println!("{}", a.iter().collect::<String>());
}
