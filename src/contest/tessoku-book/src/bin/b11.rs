use indexing::algorithms::lower_bound;
use proconio::input;
use superslice::Ext;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }
    let mut a = Vec::from(a);
    a.sort();
    for _ in 0..q {
        input! {
            x: usize,
        }
        let mut low = 0;
        let mut up = a.len();

        println!("{}", lower_bound(&a, &x));
    }
}
