use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q],
    }

    let mut prefix = vec![0; n + 1];
    prefix[1] = a[0];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + a[i];
    }

    for (l, r) in &lr {
        println!("{}", prefix[*r] - prefix[*l - 1])
    }
}
