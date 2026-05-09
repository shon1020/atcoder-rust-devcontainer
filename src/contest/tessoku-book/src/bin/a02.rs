use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    if linear_search(&a, x) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn linear_search(vec: &Vec<usize>, x: usize) -> bool {
    let mut is_ok = false;
    for &a in vec {
        if a == x {
            is_ok = true;
        }
    }
    is_ok
}
