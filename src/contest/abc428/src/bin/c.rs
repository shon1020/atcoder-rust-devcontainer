use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut s = Vec::new();
    let mut status = 0;
    let mut ng_cnt = 0;

    for _ in 0..q {
        input! {
            n: usize,
        }

        match n {
            1 => {
                input! {
                    c: char,
                }
                s.push(c);
                if c == '(' {
                    status += 1;
                } else {
                    status -= 1;
                }
                if status < 0 {
                    ng_cnt += 1;
                }
                yesno(status == 0 && ng_cnt == 0);
            }
            2 => {
                let c = s.pop().unwrap();
                if status < 0 {
                    ng_cnt -= 1;
                }
                if c == '(' {
                    status -= 1;
                } else {
                    status += 1;
                }
                yesno(status == 0 && ng_cnt == 0);
            }
            _ => (),
        }
    }
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
