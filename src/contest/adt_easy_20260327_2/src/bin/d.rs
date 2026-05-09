use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut ans_vec = vec![];
    for i in 0..n {
        let mut count = 0;
        for j in 0..n {
            if s[i][j] == 'o' {
                count += 1;
            }
        }
        ans_vec.push((count, i + 1));
    }
    ans_vec.sort_by(|a, b| b.0.cmp(&a.0));
    println!("{}", ans_vec.iter().map(|x| x.1).join(" "));
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
