/// 尺取り法: 合計が k 以上になる最短の連続部分列の長さを返す
/// 存在しない場合は None を返す
///
/// # アルゴリズム
/// - 右端 r を伸ばして sum >= k を満たしたら、左端 l を縮めて最短を探す
/// - l, r それぞれ最大 n 回移動するため O(n)
#[cargo_snippet::snippet("min_subarray_len_with_sum")]
pub fn min_subarray_len_with_sum(arr: &[i64], k: i64) -> Option<usize> {
    let n = arr.len();
    let mut l = 0;
    let mut sum = 0;
    let mut ans = usize::MAX;

    for r in 0..n {
        // 右端を伸ばす
        sum += arr[r];

        // 条件を満たす間、左端を縮めて最短を探す
        while sum >= k {
            ans = ans.min(r - l + 1);
            sum -= arr[l];
            l += 1;
        }
    }

    if ans == usize::MAX {
        None
    } else {
        Some(ans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let arr = vec![2, 3, 1, 2, 4, 3];
        // [4, 3] の長さ 2 が最短
        assert_eq!(min_subarray_len_with_sum(&arr, 7), Some(2));
    }

    #[test]
    fn test_entire_array() {
        // 配列全体でちょうど k を満たす
        let arr = vec![1, 1, 1, 1];
        assert_eq!(min_subarray_len_with_sum(&arr, 4), Some(4));
    }

    #[test]
    fn test_single_element() {
        let arr = vec![5];
        assert_eq!(min_subarray_len_with_sum(&arr, 5), Some(1));
    }

    #[test]
    fn test_not_found() {
        // 合計が k に届かない
        let arr = vec![1, 1, 1];
        assert_eq!(min_subarray_len_with_sum(&arr, 10), None);
    }
}

// Union Findを実装
//とりあえず、経路圧縮は使わずに書く
pub struct UnionFind {
    pair: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            pair: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        let mut x = x.clone();

        loop {
            if self.pair[x] == x {
                break;
            } else {
                x = self.pair[x];
            }
        }
        x
    }

    fn unite(&mut self, u: usize, v: usize) {
        let root_u = self.root(u);
        let root_v = self.root(v);
        if root_u == root_v {
            return;
        }
        if self.size[root_u] < self.size[root_v] {
            self.pair[root_u] = root_v;
            self.size[root_v] += self.size[root_u];
        } else {
            self.pair[root_v] = root_u;
            self.size[root_u] += self.size[root_v];
        }
    }

    fn issame(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }
}
