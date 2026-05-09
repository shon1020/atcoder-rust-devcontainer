use itertools::Itertools;
use proconio::input;
use std::cmp::max;
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut a = Vec::from(a);
    let mut b = Vec::from(b);
    let mut prefix_a: Vec<usize> = vec![0; n + 1];
    let mut prefix_b: Vec<usize> = vec![0; n + 1];

    for i in 0..n {
        prefix_a[i + 1] = prefix_a[i] + a[i];
        prefix_b[i + 1] = prefix_b[i] + b[i];
    }

    // 解がない場合も考えないといけなかった。
    let mut ans = 0;
    for i in 1..=n {
        let mut low = i - 1; // 「有効な区間なし」を表す
        let mut up = n;
        while low < up {
            let mid = (low + up + 1) / 2;
            if prefix_b[mid] - prefix_b[i - 1] <= k {
                low = mid;
            } else {
                up = mid - 1;
            }
            if low >= i {
                ans = max(ans, prefix_a[low] - prefix_a[i - 1]);
            }
        }
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
