use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut ev = vec![0; d + 2];
    for (l, r) in &lr {
        ev[*l] += 1;
        ev[*r + 1] -= 1;
    }
    for i in 0..d {
        ev[i + 1] = ev[i] + ev[i + 1]
    }

    for i in 1..(d + 1) {
        println!("{}", ev[i])
    }
}
