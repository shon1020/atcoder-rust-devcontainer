use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let mut grid = Vec::new();

    for i in 1..n {
        input! {
            vec: [usize; n - i],
        }
        let mut new_vec = vec![0; i];
        for v in vec {
            new_vec.push(v);
        }
        grid.push(new_vec);
    }
    let mut is_ok = false;
    for a in 0..n - 2 {
        for b in a + 1..n - 1 {
            for c in b + 1..n {
                if grid[a][c] > (grid[a][b] + grid[b][c]) {
                    is_ok = true;
                }
            }
        }
    }
    yesno(is_ok);
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
