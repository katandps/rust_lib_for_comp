//! # 文字列の繰り返し
//! 同じ文字列を繰り返したものから文字を検索する

use crate::algo::slice_bounds::SliceBounds;

#[codesnip::entry("repeat-string")]
pub use repeat_string_impl::{RepeatString, RepeatStringCursor};
#[codesnip::entry("repeat-string", include("slice-bounds"))]
mod repeat_string_impl {
    use super::SliceBounds;
    #[derive(Clone, Debug)]
    pub struct RepeatString<T> {
        pub src: Vec<T>,
        /// srcにおける各charの出現位置
        pos: Vec<Vec<usize>>,
    }

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct RepeatStringCursor {
        pub rep: usize,
        pub next_pos: usize,
    }

    impl<T: Into<usize> + Clone> RepeatString<T> {
        /// # 生成
        ///
        pub fn build(src: &[T]) -> Self {
            let src = src.to_vec();
            // pos := sにおけるalphabetの出現位置
            let mut pos = vec![Vec::new(); 27];
            for i in 0..src.len() {
                pos[src[i].clone().into()].push(i);
            }
            Self { src, pos }
        }

        /// # cursor以降でcnt個先のcharの位置を返す
        pub fn next<C: Into<usize>>(
            &self,
            cnt: C,
            char: T,
            mut cursor: RepeatStringCursor,
        ) -> Option<RepeatStringCursor> {
            let mut cnt = cnt.into();
            let p: usize = char.into();
            if self.pos[p].is_empty() {
                return None;
            }
            let one_loop = self.pos[p].len();
            if self.pos[p][self.pos[p].len() - 1] < cursor.next_pos {
                cursor.next_pos = 0;
                cursor.rep += 1;
            }
            let next = self.pos[p].lower_bound(&cursor.next_pos);
            if next == one_loop {
            } else {
                cursor.next_pos = self.pos[p][next] + 1;
                cnt -= 1;
            }
            if cnt > 0 {
                if cnt > one_loop {
                    cursor.rep += cnt / one_loop;
                    cnt %= one_loop;
                }
                if cnt > 0 {
                    let next = self.pos[p].lower_bound(&cursor.next_pos);
                    let rest_right = one_loop - next;
                    if rest_right < cnt {
                        cursor.rep += 1;
                        cnt -= rest_right;
                        cursor.next_pos = self.pos[p][cnt - 1] + 1;
                    } else {
                        cursor.next_pos = self.pos[p][next + cnt - 1] + 1;
                    }
                }
            }
            Some(cursor)
        }
    }

    impl RepeatStringCursor {
        /// # rep回繰り返した文字列より手前にあるか
        pub fn before(&self, rep: usize) -> bool {
            self.rep < rep || (self.rep == rep && self.next_pos == 0)
        }
    }
}
