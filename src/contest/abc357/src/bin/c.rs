use itertools::Itertools;
use num_traits::pow;
use proconio::input;
fn main() {
    input! {
       n: usize,
    }
    let mut grid = vec![vec!["#"; pow(3, n)]; pow(3, n)];

    for level in 0..(n + 1) {
        let size = pow(3, level);
        let cnt = pow(3, n) / size;
        let sub = size / 3;
        for x in 0..cnt {
            for y in 0..cnt {
                // レベル = levelのカーペットのうち
                // 上からx番目、左からy番目
                let xi = size * x;
                let yj = size * y;

                let si = xi + sub;
                let sj = yj + sub;

                for i in si..si + sub {
                    for j in sj..sj + sub {
                        grid[i][j] = ".";
                    }
                }
            }
        }
    }
    for ans in grid {
        println!("{}", ans.iter().map(|x| x.to_string()).join(""));
    }
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
