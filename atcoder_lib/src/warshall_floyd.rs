#[allow(unused_imports)]
use warshall_floyd::*;

#[allow(dead_code)]
mod warshall_floyd {
    use std::cmp::min;

    type COST = usize;
    type EDGE = (usize, usize, COST);

    ///
    /// 辺の情報からWarshallFloyd法により全点間最小コストを計算する
    /// 計算量 O(N^3)
    ///
    pub fn warshall_floyd(vertex_n: usize, edges: &Vec<EDGE>) -> Vec<Vec<COST>> {
        let mut ret = vec![vec![1_000_000_000usize; vertex_n + 1]; vertex_n + 1];
        for i in 0..vertex_n + 1 {
            ret[i][i] = 0;
        }
        for &(a, b, cost) in edges {
            ret[a][b] = min(ret[a][b], cost);
            ret[b][a] = min(ret[b][a], cost); //有向グラフの場合はコメントアウト
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
        let edges = vec![(1, 2, 1), (2, 3, 10), (3, 1, 100)];

        let wf = warshall_floyd(3, &edges);
        assert_eq!(wf[1][2], 1);
        assert_eq!(wf[1][3], 11);
        assert_eq!(wf[2][3], 10);
    }
}
