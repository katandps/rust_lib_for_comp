#[allow(unused_imports)]
use warshall_floyd::*;

#[allow(dead_code)]
mod warshall_floyd {
    use std::cmp::min;

    pub fn warshall_floyd(
        vertex_n: usize,
        edge_n: usize,
        a: Vec<usize>,
        b: Vec<usize>,
        cost: Vec<usize>,
    ) -> Vec<Vec<usize>> {
        let mut ret = vec![vec![1_000_000_000usize; vertex_n + 1]; vertex_n + 1];
        for i in 0..vertex_n + 1 {
            ret[i][i] = 0;
        }
        for i in 0..edge_n {
            ret[a[i]][b[i]] = min(ret[a[i]][b[i]], cost[i]);
            ret[b[i]][a[i]] = min(ret[b[i]][a[i]], cost[i]); //有向グラフの場合はコメントアウト
        }
        for i in 0..vertex_n + 1 {
            for j in 0..vertex_n + 1 {
                for k in 0..vertex_n + 1 {
                    ret[j][k] = min(ret[j][k], ret[j][i] + ret[i][k])
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let wf = warshall_floyd(3, 3, vec![1, 2, 3], vec![2, 3, 1], vec![1, 10, 100]);
        assert_eq!(wf[1][2], 1);
        assert_eq!(wf[1][3], 11);
        assert_eq!(wf[2][3], 10);
    }
}
