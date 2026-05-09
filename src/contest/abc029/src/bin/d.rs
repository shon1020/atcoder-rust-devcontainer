use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans: usize = 0;

    println!("{}", ans);
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    }
    else {
        println!("No")
    }
}
