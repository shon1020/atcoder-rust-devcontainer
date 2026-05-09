use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let a = Vec::from(a);
    let mut low: usize = 1;
    let mut up: usize = 100_000_000_000;

    while low < up {
        let mid = (low + up) / 2;
        if check(&a, k, mid) {
            up = mid;
        } else {
            low = mid + 1;
        }
    }
    println!("{}", low);
}

fn check(vec: &Vec<usize>, k: usize, t: usize) -> bool {
    let mut tol = 0;
    for i in 0..vec.len() {
        tol += t / vec[i];
    }
    if tol >= k {
        true
    } else {
        false
    }
}
