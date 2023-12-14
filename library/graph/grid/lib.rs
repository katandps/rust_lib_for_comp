//! グリッドグラフ
use graph::GraphTrait;
use prelude::*;

#[codesnip::entry("grid-graph", doc_hidden)]
#[derive(Debug)]
pub struct Grid<W> {
    pub h: usize,
    pub w: usize,
    pub size: usize,
    pub map: Vec<W>,
}

#[codesnip::entry("grid-graph", doc_hidden)]
impl<W: Clone> GraphTrait for Grid<W> {
    type Weight = ();

    fn size(&self) -> usize {
        self.size
    }

    fn edges(&self, src: usize) -> Vec<(usize, ())> {
        let mut ret = Vec::with_capacity(4);
        if self.x(src) + 1 < self.w {
            ret.push((src + 1, ()));
        }
        if self.y(src) + 1 < self.h {
            ret.push((src + self.w, ()));
        }
        if self.x(src) > 0 {
            ret.push((src - 1, ()));
        }
        if self.y(src) > 0 {
            ret.push((src - self.w, ()));
        }
        ret
    }

    fn rev_edges(&self, dst: usize) -> Vec<(usize, ())> {
        let mut ret = Vec::with_capacity(4);
        if self.x(dst) + 1 < self.w {
            ret.push((dst + 1, ()));
        }
        if self.y(dst) + 1 < self.h {
            ret.push((dst + self.w, ()));
        }
        if self.x(dst) > 0 {
            ret.push((dst - 1, ()));
        }
        if self.y(dst) > 0 {
            ret.push((dst - self.w, ()));
        }
        ret
    }
}

#[codesnip::entry("grid-graph", doc_hidden)]
impl<W: Clone> Grid<W> {
    pub fn new(h: usize, w: usize, input: Vec<Vec<W>>) -> Grid<W> {
        let mut map = Vec::new();
        for r in input {
            for c in r {
                map.push(c);
            }
        }
        let max = h * w;
        Grid {
            h,
            w,
            size: max,
            map,
        }
    }
    pub fn key(&self, x: usize, y: usize) -> usize {
        y * self.w + x
    }
    pub fn xy(&self, k: usize) -> (usize, usize) {
        (self.x(k), self.y(k))
    }
    pub fn x(&self, k: usize) -> usize {
        k % self.w
    }
    pub fn y(&self, k: usize) -> usize {
        k / self.w
    }
    pub fn get(&self, key: usize) -> &W {
        &self.map[key]
    }
    pub fn set(&mut self, key: usize, value: W) {
        self.map[key] = value;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let grid = Grid::new(
            3,
            3,
            vec![vec![10, 20, 30], vec![40, 50, 60], vec![70, 80, 90]],
        );

        let (x, y) = (1, 1);
        let key = grid.key(x, y);
        assert_eq!(grid.get(key), &50);
        let e = grid.edges(0);
        let e1 = e[0];
        let e2 = e[1];
        assert_eq!(1, e1.0);
        assert_eq!(3, e2.0);
    }
}
