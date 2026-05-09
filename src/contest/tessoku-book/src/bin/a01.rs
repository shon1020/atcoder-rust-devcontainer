use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans: usize = n.pow(2);

    println!("{}", ans);
}
