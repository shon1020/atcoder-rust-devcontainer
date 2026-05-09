use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    if n == 1 && m == 7 {
        println!("Yes");
    } else if n == 3 && m == 3 {
        println!("Yes");
    } else if n == 5 && m == 5 {
        println!("Yes");
    } else if n == 7 && m == 7 {
        println!("Yes");
    } else if n == 9 && m == 9 {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
