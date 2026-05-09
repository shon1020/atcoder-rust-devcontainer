use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let mut g_pre = vec![0; n + 1];
    let mut b_pre = vec![0; n + 1];

    for i in 0..n {
        if a[i] == 1 {
            g_pre[i + 1] = g_pre[i] + 1;
        } else {
            g_pre[i + 1] = g_pre[i]
        }
    }

    for i in 0..n {
        if a[i] == 0 {
            b_pre[i + 1] = b_pre[i] + 1;
        } else {
            b_pre[i + 1] = b_pre[i]
        }
    }

    for (l, r) in &lr {
        let g = g_pre[*r] - g_pre[*l - 1];
        let b = b_pre[*r] - b_pre[*l - 1];
        if g > b {
            println!("win");
        } else if g == b {
            println!("draw");
        } else {
            println!("lose");
        }
    }
}
