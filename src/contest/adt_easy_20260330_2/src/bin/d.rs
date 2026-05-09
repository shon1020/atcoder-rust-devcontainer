use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut vec = Vec::new();
    for (i, v) in p.iter().enumerate() {
        vec.push((v, i));
    }
    vec.sort();
    let mut ans_vec = vec![0; n];
    let mut r = 1;
    while vec.len() != 0 {
        let x = vec[vec.len() - 1].0;
        let mut k = 0;
        let mut q = vec![];
        while vec.len() != 0 && vec[vec.len() - 1].0 == x {
            let idx = vec.pop().unwrap().1;
            q.push(idx);
            k += 1;
        }
        for i in q {
            ans_vec[i] = r;
        }
        r += k;
    }
    for ans in ans_vec {
        println!("{}", ans);
    }
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
