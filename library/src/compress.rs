/// 座標圧縮 O(NlogN)
#[allow(dead_code)]
fn compress<T: Ord>(source: &[T]) -> Vec<usize> {
    let n = source.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by_key(|&i| &source[i]);
    let mut ret = vec![0; n];
    let mut cur = 0;
    for i in 0..n {
        if i > 0 && source[idx[i - 1]] != source[idx[i]] {
            cur += 1;
        }
        ret[idx[i]] = cur;
    }
    ret
}

//-----------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::compress::*;

    #[test]
    fn compress_test() {
        let s = vec![0, 10, 100, 50, 5, 2];
        let r = compress(&s);
        assert_eq!(r, vec![0, 3, 5, 4, 2, 1]);
    }
}
