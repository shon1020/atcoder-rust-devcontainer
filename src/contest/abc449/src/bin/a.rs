use proconio::input;
use std::f64::consts::PI;
fn main() {
    input! {
        d: f64,
    }

    println!("{}", (d / 2.0) * (d / 2.0) * PI);
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
