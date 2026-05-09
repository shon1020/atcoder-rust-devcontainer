use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans: String = "".to_string();
    match n % 2 {
        1 => {
            for i in 0..n {
                if i == (n / 2 + 1) {
                    ans.push_str("=")
                } else {
                    ans.push_str("-")
                };
            }
        }
        2 => {
            for i in 0..n {
                if i == n / 2 || i == n / 2 + 1 {
                    ans.push_str("=");
                } else {
                    ans.push_str("-");
                }
            }
        }
        _ => (),
    }
    println!("{}", ans);
}
