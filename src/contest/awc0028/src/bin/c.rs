use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i64; n],
        mut a: [i64; n],
        b: [i64; n],
    }
    let mut cad_idx = 0;
    let mut cad_max = -10_i64.pow(10);
    for (idx, v) in a.iter().enumerate() {
        if p[idx] - *v > cad_max {
            cad_idx = idx;
            cad_max = p[idx] - v;
        }
    }

    let mut ans: i64 = 0;
    a[cad_idx] = p[cad_idx];
    let mut ans: i64 = 0;
    for i in 0..n {
        ans += a[i] - b[i];
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
