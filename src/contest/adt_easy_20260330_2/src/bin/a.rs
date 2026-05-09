use itertools::Itertools;
use num_traits::pow;
use proconio::input;
fn main() {
    input! {
        x: usize,
        y: usize,
        n: usize,
    }

    let mut ans = pow(10, 9);
    let mut b = 0;
    while 3 * b <= n {
        let cad = x * (n - 3 * b) + y * b;
        if cad < ans {
            ans = cad;
        }
        b += 1;
    }
    println!("{}", ans);
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
