use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[i64; w]; h],
        q: usize
    }

    let mut grid = vec![vec![0; w + 2]; h + 2];
    for i in 0..h {
        for j in 0..w {
            grid[i + 1][j + 1] = x[i][j];
        }
    }
    for i in 0..(w + 1) {
        for j in 0..h {
            grid[j + 1][i] += grid[j][i];
        }
    }
    for i in 0..(h + 1) {
        for j in 0..w {
            grid[i][j + 1] += grid[i][j];
        }
    }

    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        }
        println!(
            "{}",
            grid[c][d] + grid[a - 1][b - 1] - grid[a - 1][d] - grid[c][b - 1]
        );
    }
}
