use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut is_ok = false;

    for i in 0..(n - 2) {
        for j in (i + 1)..(n - 1) {
            for k in (j + 1)..n {
                if a[i] + a[j] + a[k] == 1000 {
                    is_ok = true;
                }
            }
        }
    }
    yesno(is_ok);
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
