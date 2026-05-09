use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut a = Vec::from(a);
    a.sort();
    let mut a_rlc = rlc(&a);
    let mut ans_vec = Vec::new();
    for (num, c) in a_rlc {
        ans_vec.push(num * c);
    }
    ans_vec.sort();
    for i in 0..k {
        if ans_vec.len() == 0 {
            break;
        }
        ans_vec.pop();
    }
    let mut ans = 0;
    if ans_vec.len() == 0 {
        println!("{}", ans);
    } else {
        while ans_vec.len() > 0 {
            ans += ans_vec.pop().unwrap();
        }
        println!("{}", ans);
    }
}

fn rlc(a: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut i = 0;
    let mut ctr = vec![];
    let mut cur = (a[0], 0);
    loop {
        while i < a.len() && a[i] == cur.0 {
            cur.1 += 1;
            i += 1;
        }
        ctr.push((cur));
        if i == a.len() {
            break;
        } else {
            cur = (a[i], 0);
        }
    }
    ctr
}
