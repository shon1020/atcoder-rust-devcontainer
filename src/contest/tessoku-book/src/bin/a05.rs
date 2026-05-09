use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
    }

    let mut count: i64 = 0;
    for cr in 1..(n + 1) {
        for cb in 1..(n + 1) {
            let cw = k - &cr - &cb;
            if 1 <= cw && cw <= n {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
