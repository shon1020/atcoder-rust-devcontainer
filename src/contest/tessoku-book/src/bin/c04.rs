use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut vec = vec![];
    let mut i = 1;
    while *&(i * i) < n {
        if n % i == 0 {
            vec.push(i);
            if i != n / i {
                vec.push((n / i));
            }
        }
        i += 1;
    }
    vec.sort();

    for v in vec {
        println!("{}", v);
    }
}
