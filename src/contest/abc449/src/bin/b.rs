use proconio::input;

fn main() {
    input! {
        mut h: usize,
        mut w: usize,
        q: usize,
    }

    let mut grid = vec![vec![true; w]; h];

    for _ in 0..q {
        input! {
            n: usize,
        }

        match n {
            1 => {
                input! {
                    r: usize,
                }
                let mut count = 0;
                for i in (h - r)..h {
                    for j in 0..w {
                        if grid[i][j] {
                            count += 1;
                            grid[i][j] == false;
                        }
                    }
                }
                h = h - r;
                println!("{}", count);
            }
            2 => {
                input! {
                    c: usize,
                }
                let mut count = 0;
                for i in 0..h {
                    for j in (w - c)..w {
                        if grid[i][j] {
                            count += 1;
                            grid[i][j] = false;
                        }
                    }
                }
                println!("{}", count);
                w -= c;
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
