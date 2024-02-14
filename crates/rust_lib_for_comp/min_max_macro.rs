//! 最大最小を求めるマクロ

#[codesnip::entry("min")]
#[allow(unused_macros)]
#[macro_export]
macro_rules! min {
    ($a:expr $(,)*) => {{$a}};
    ($a:expr, $b:expr $(,)*) => {
        {
            let (ar, br) = ($a, $b);
            if ar > br {br} else {ar}
        }
    };
    ($a:expr, $($rest:expr),+ $(,)*) => {
        {
            let b = min!($($rest),+);
            let ar = $a;
            if ar > b {b} else {ar}
        }
    };
}
#[codesnip::entry("chmin", include("min"))]
#[allow(unused_macros)]
#[macro_export]
macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {
        {
            let cmp_min = min!($($cmps),+);
            if $base > cmp_min {
                $base = cmp_min;
                true
            } else {
                false
            }
        }
    };
}
#[codesnip::entry("max")]
#[allow(unused_macros)]
#[macro_export]
macro_rules! max {
    ($a:expr $(,)*) => {{$a}};
    ($a:expr, $b:expr $(,)*) => {
        {
            let (ar, br) = ($a, $b);
            if ar > br {ar} else {br}
        }
    };
    ($a:expr, $($rest:expr),+ $(,)*) => {
        {
            let b = max!($($rest),+);
            let ar = $a;
            if ar > b {ar} else {b}
        }
    };
}
#[codesnip::entry("chmax", include("max"))]
#[allow(unused_macros)]
#[macro_export]
macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {
        {
            let cmp_max = max!($($cmps),+);
            if $base < cmp_max {
                $base = cmp_max;
                true
            } else {
                false
            }}
    };
}

#[codesnip::entry("clamp", include("min", "max"))]
#[allow(unused_macros)]
#[macro_export]
macro_rules! clamp {
    ($base: expr, $lower_bound: expr, $upper_bound: expr) => {
        max!($lower_bound, min!($upper_bound, $base))
    };
}

#[codesnip::entry("chclamp", include("min", "max"))]
#[allow(unused_macros)]
#[macro_export]
macro_rules! chclamp {
    ($base: expr, $lower_bound: expr, $upper_bound: expr) => {
        chmin!($base, $upper_bound) || chmax!($base, $lower_bound)
    };
}

#[allow(unused_imports)]
pub(crate) use {chclamp, chmax, chmin, clamp, max, min};

#[test]
fn test() {
    // min
    let mut ans = 42;
    let changed = chmin!(ans, 100, 0, -5, 100 * 2, 100 / 2);
    assert_eq!(ans, -5);
    assert!(changed);

    let mut ans = -10;
    let changed = chmin!(ans, 100, 0, -5, 100 * 2, 100 / 2, -10);
    assert_eq!(ans, -10);
    assert!(!changed);

    // max
    let mut ans = 42;
    let changed = chmax!(ans, 100, 0, -5, 100 * 2, 100 / 2);
    assert_eq!(ans, 200);
    assert!(changed);

    let mut ans = 201;
    let changed = chmax!(ans, 100, 0, -5, 100 * 2, 100 / 2, 201);
    assert_eq!(ans, 201);
    assert!(!changed);

    let mut ans = 0.50;
    let changed = chmax!(ans, 0.50, 1.0 / 2.0, 0.49, 0.25 * 2.0);
    assert_eq!(ans, 0.50);
    assert!(!changed);

    let mut ans = 0.50;
    let changed = chmax!(ans, 0.50, 0.51, 0.50 + 0.1);
    assert_eq!(ans, 0.6);
    assert!(changed);
}

#[test]
fn test_clamp() {
    assert_eq!(100, clamp!(200, 0, 100));
    assert_eq!(0, clamp!(-100, 0, 100));
    assert_eq!(50, clamp!(50, 0, 100));

    let mut val = 200;
    let b = chclamp!(val, 0, 100);
    assert!(b);
    assert_eq!(val, 100);

    let mut val = -100;
    let b = chclamp!(val, 0, 100);
    assert!(b);
    assert_eq!(val, 0);

    let mut val = 50;
    let b = chclamp!(val, 0, 100);
    assert!(!b);
    assert_eq!(val, 50);
}
