use proconio::input;

fn main() {
    input! {
        h: usize,
    }

    let mut ans = 0;
    ans = monster(&h);
    println!("{}", ans);
}

fn monster(h: &usize) -> usize {
    if *h == 1 {
        return 1;
    }
    let div = h / 2;
    return monster(&div) * 2 + 1;
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
