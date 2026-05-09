use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        n: Chars,
    }

    for i in 1..=2 {
        print!("{}", n[i as usize]);
    }
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
