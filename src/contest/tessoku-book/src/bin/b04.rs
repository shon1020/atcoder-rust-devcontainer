use num_traits::pow;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        n :Chars,
    }
    let mut ans: usize = 0;

    for i in 0..n.len() {
        if n[i] == '1' {
            ans += pow(2, n.len() - 1 - i);
        }
    }
    println!("{}", ans);
}
