#[allow(unused_imports)]
use graph::*;

#[allow(dead_code)]
pub mod graph {
    use std::cmp::Ordering;

    pub type Weight = i64;

    #[derive(Copy, Clone)]
    pub struct Edge {
        pub src: i64,
        pub dst: i64,
        pub weight: Weight,
    }

    impl Edge {
        pub fn default() -> Edge {
            let (src, dst, weight) = (0, 0, 0);
            Edge { src, dst, weight }
        }

        pub fn new(src: i64, dst: i64, weight: Weight) -> Edge {
            Edge { src, dst, weight }
        }
    }

    impl std::cmp::PartialEq for Edge {
        fn eq(&self, other: &Self) -> bool {
            self.weight.eq(&other.weight)
        }
    }

    impl std::cmp::Eq for Edge {}

    impl std::cmp::PartialOrd for Edge {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.weight.partial_cmp(&other.weight)
        }
    }

    impl std::cmp::Ord for Edge {
        fn cmp(&self, other: &Self) -> Ordering {
            self.weight.cmp(&other.weight)
        }
    }

    #[derive(Clone)]
    pub struct Edges(pub Vec<Edge>);

    pub struct Graph(pub Vec<Edges>);

    pub struct Array(Vec<Weight>);

    pub struct Matrix(Vec<Array>);

    impl Graph {
        pub fn from_matrix(weights: &Vec<Vec<Weight>>, n: usize) -> Graph {
            let mut ret = Graph(vec![Edges(Vec::new()); n]);
            for i in 0..n as i64 {
                for j in i + 1..n as i64 {
                    if weights[i as usize][j as usize] == -1 {
                        continue;
                    }
                    ret.add_edge(i, j, weights[i as usize][j as usize]);
                    ret.add_edge(j, i, weights[j as usize][i as usize]);
                }
            }
            ret
        }

        pub fn len(&self) -> usize {
            self.0.len()
        }

        pub fn add_edge(&mut self, a: i64, b: i64, w: Weight) {
            self.0[a as usize].0.push(Edge::new(a, b, w));
            self.0[b as usize].0.push(Edge::new(b, a, w));
        }

        pub fn add_arc(graph: &mut Graph, a: i64, b: i64, w: Weight) {
            graph.0[a as usize].0.push(Edge::new(a, b, w));
        }
    }
}
