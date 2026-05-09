use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }

    let mut ans_idx = 0;
    let mut v_max = 0;

    for (i, v) in g.iter().enumerate() {
        let len = v.len();
        if v_max < len {
            ans_idx = i + 1;
            v_max = len;
        }
    }

    println!("{}", ans_idx);
}
