use pathfinding::matrix::directions::W;
use proconio::{input, marker::Chars};
use superslice::*;
// このやり方はダメ！！
// fn main() {
//     input! {
//         n: usize,
//         l: usize,
//         r: usize,
//         s: Chars,
//     }

//     let mut vec = vec![0; 26];
//     let mut ans: usize = 0;
//     for i in 0..n {
//         if i >= l {
//             vec[s[i - l] as usize - 'a' as usize] += 1;
//         }
//         if i >= r + 1 {
//             vec[s[i - r - 1] as usize - 'a' as usize] -= 1;
//         }

//         ans += vec[s[i] as usize - 'a' as usize];
//     }
//     println!("{}", ans);
// }
fn main() {
    input! {
     n: usize,
     l: usize,
     r: usize,
     s: Chars,
    }
    //各文字ごとに出現するインデックスを記録するベクター
    let mut vec = vec![Vec::new(); 26];
    for (i, st) in s.iter().enumerate() {
        vec[*st as usize - 'a' as usize].push(i);
    }

    for i in 0..n {}

    let mut ans = 0;
    for i in 0..n {
        let st = s[i];
        ans += vec[st as usize - 'a' as usize].upper_bound(&(i + r))
            - vec[st as usize - 'a' as usize].lower_bound(&(i + l));
    }

    println!("{}", ans);
}
fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
