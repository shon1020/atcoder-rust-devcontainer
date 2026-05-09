use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
        t: Chars
    }

    let mut cnt = 0;
    for i in 0..n {
        if s[i] != t[i] {
            cnt += 1;
        }
    }
    println!("{}", if cnt > k { cnt - k } else { 0 });
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
