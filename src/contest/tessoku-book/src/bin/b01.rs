use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let ans: usize = a + b;

    println!("{}", ans);
}
