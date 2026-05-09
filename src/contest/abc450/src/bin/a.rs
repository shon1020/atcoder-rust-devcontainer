use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    for i in (1..n + 1).rev() {
        if i == 1 {
            print!("{}", i);
        } else {
            print!("{},", i);
        }
    }
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
