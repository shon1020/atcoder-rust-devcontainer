use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            mut s: [usize; n],
        }

        let start = s[0];
        let end = s[n - 1];

        s.sort_unstable();

        if end <= 2 * start {
            println!("2");
            continue;
        }

        let mut ans = 1;
        let mut cur = start;
        let mut is_ok = true;

        loop {
            let next = s[s.partition_point(|&x| x <= 2 * cur) - 1];

            if next <= cur {
                is_ok = false;
                break;
            }

            ans += 1;
            cur = next;

            if end <= 2 * cur {
                ans += 1;
                break;
            }
        }

        if is_ok {
            println!("{}", ans);
        } else {
            println!("-1");
        }
    }
}
