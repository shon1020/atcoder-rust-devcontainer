use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s: Chars,
    }

    println!("{}", if s.len() % 5 == 0 { "Yes" } else { "No" });
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
