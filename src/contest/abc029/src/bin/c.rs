use proconio::input;

const DICT: [&str; 3] = ["a", "b", "c"];

fn main() {
    input! {
        n: usize,
    }
    let mut s: String = "".to_string();
    let mut ans_vec = vec![];
    bfa(n, &s, &mut ans_vec);
    for ans in &ans_vec {
        println!("{}", ans);
    }
}
fn bfa(n: usize, s: &String, vec: &mut Vec<String>) {
    if n == 0 {
        return vec.push(s.to_string());
    }
    for c in DICT {
        let st: String = s.to_string() + c;
        bfa(n - 1, &st, vec);
    }
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
