use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }

    let mut ans: usize = 0;

    for v in &p {
        if *v >= k {
            ans += v;
        }
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
