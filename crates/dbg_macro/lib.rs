//! # debugマクロ
//! releaseビルドではdbg!を無効にする

use prelude::*;

#[snippet("debug_macro")]
#[allow(unused_macros)]
macro_rules! dbg {
    ($($x:tt)*) => {
        {
            #[cfg(debug_assertions)]
            {
                std::dbg!($($x)*)
            }
            #[cfg(not(debug_assertions))]
            {
                ($($x)*)
            }
        }
    }
}
