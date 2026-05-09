use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    let mut a = Vec::from(a);
    a.sort();
    let ans = binary_search(a, x);
    if ans != -1 {
        println!("{}", ans);
    }
}

fn binary_search(a: Vec<usize>, target: usize) -> i64 {
    let mut left = 0;
    let mut right = a.len() - 1;
    let mut idx: i64 = a.len() as i64;
    let mut is_ok = false;
    while left <= right {
        let mut mid = (right + left) / 2;
        if a[mid] == target {
            idx = mid as i64;
            is_ok = true;
            break;
        } else if a[mid] > target {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    if is_ok {
        idx + 1
    } else {
        -1
    }
}
