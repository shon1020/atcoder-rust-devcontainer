use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut p: Vec<i64> = vec![0; t + 1];
    let mut ans: usize = 0;

    for (l, r) in &lr {
        p[*l] += 1;
        p[*r] -= 1;
    }

    for i in 0..t {
        p[i + 1] = p[i] + p[i + 1];
    }

    for i in 0..t {
        println!("{}", p[i])
    }
}
