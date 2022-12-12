//! # 完全64分木
//! $n<2^{18} = 262143$の範囲でsetのようなInterfaceを提供する 高速・省メモリ
//!
//! 集合に存在するかどうかをBitSetの入れ子のような形で表現する

use crate::prelude::*;

#[snippet(name = "complete_64_part_tree", doc_hidden)]
pub use complete_64_part_tree_impl::Complete64PartTree;

#[snippet(name = "complete_64_part_tree", doc_hidden)]
mod complete_64_part_tree_impl {

    #[derive(Clone)]
    pub struct Complete64PartTree {
        pub map: Vec<Vec<u64>>,
    }

    impl Default for Complete64PartTree {
        fn default() -> Self {
            Self {
                map: vec![vec![0], vec![0; 1 << 6], vec![0; 1 << 12]],
            }
        }
    }

    impl Complete64PartTree {
        /// # keyを追加する
        /// すでに存在していた時はfalse
        pub fn push(&mut self, key: usize) -> bool {
            assert!(key < 1 << 18);
            if self.map[2][key >> 6] & (1 << (key & 63)) > 0 {
                return false;
            }
            self.map[2][key >> 6] |= 1 << (key & 63);
            self.map[1][key >> 12] |= 1 << (key >> 6 & 63);
            self.map[0][0] |= 1 << (key >> 12);
            true
        }

        /// # keyを消す
        /// 存在していたときはtrue
        pub fn erase(&mut self, key: usize) -> bool {
            assert!(key < 1 << 18);
            if self.map[2][key >> 6] & (1 << (key & 63)) == 0 {
                return false;
            }
            self.map[2][key >> 6] &= !(1 << (key & 63));
            if self.map[2][key >> 6] == 0 {
                self.map[1][key >> 12] &= !(1 << (key >> 6 & 63));
                if self.map[1][key >> 12] == 0 {
                    self.map[0][0] &= !(1 << (key >> 12));
                }
            }
            true
        }

        /// # 値の存在判定
        pub fn contains(&self, key: usize) -> bool {
            assert!(key < 1 << 18);
            self.map[2][key >> 6] & (1 << (key & 63)) > 0
        }

        /// # 最大値を返す
        pub fn max(&self) -> Option<usize> {
            let mut ret = None;
            let mut pos = 0;

            for b in 0..3 {
                if self.map[b][pos] == 0 {
                    break;
                }
                pos = pos << 6 | self.map[b][pos as usize].trailing_zeros() as usize;
                ret = Some(pos);
            }
            ret
        }

        /// # 最大値を消費して返す
        pub fn pop_max(&mut self) -> Option<usize> {
            let ret = self.max();
            if let Some(r) = ret {
                self.erase(r);
            }
            ret
        }

        /// # 最小値を返す
        pub fn min(&self) -> Option<usize> {
            todo!()
        }

        /// # 最小値を消費して返す
        pub fn pop_min(&mut self) -> Option<usize> {
            let ret = self.min();
            if let Some(r) = ret {
                self.erase(r);
            }
            ret
        }
    }
}

#[cfg(test)]
mod test {
    use super::Complete64PartTree;

    #[test]
    fn test() {
        let mut tree = Complete64PartTree::default();
        assert!(tree.push(5));
        assert_eq!(Some(5), tree.max());
        tree.erase(5);
        assert_eq!(None, tree.max());
        tree.push(5);
        assert!(!tree.push(5));
        assert!(tree.push(7));

        assert!(tree.push(15));
        assert_eq!(Some(5), tree.max());
        assert!(tree.push(3));
        assert_eq!(Some(3), tree.max());
        assert!(tree.erase(3));
        assert!(!tree.erase(3));
        assert!(tree.erase(5));
        assert_eq!(Some(7), tree.max());
        assert!(tree.erase(7));
        assert!(tree.erase(15));
        assert_eq!(None, tree.max());
    }
}
