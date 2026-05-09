use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
    }

    let mut ans_vec = Vec::new();
    rec(n, &mut ans_vec);
    println!(
        "{}",
        ans_vec
            .into_iter()
            .map(|x| x.to_string())
            .collect_vec()
            .join(" ")
    );
}
fn rec(n: usize, vec: &mut Vec<usize>) {
    if n == 1 {
        vec.push(1);
        return;
    }
    rec(n - 1, vec);
    vec.push(n);
    rec(n - 1, vec);
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
