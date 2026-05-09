use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut ans = 0;

    for h_1 in 0..h {
        for h_2 in h_1..h {
            for w_1 in 0..w {
                for w_2 in w_1..w {
                    let mut is_ok = true;
                    for i in h_1..=h_2 {
                        for j in w_1..=w_2 {
                            if s[i][j] != s[h_1 + h_2 - i][w_1 + w_2 - j] {
                                is_ok = false;
                            }
                        }
                    }
                    if is_ok {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
