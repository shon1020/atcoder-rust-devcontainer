use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut is_ok = false;
    for i in a..(b + 1) {
        if 100 % i == 0 {
            is_ok = true;
        }
    }
    yesno(is_ok);
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
