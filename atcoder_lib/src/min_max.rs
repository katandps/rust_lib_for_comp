macro_rules! chmin {($base:expr, $($cmps:expr),+ $(,)*) => {{let cmp_min = min!($($cmps),+);if $base > cmp_min {$base = cmp_min;true} else {false}}};}
macro_rules! chmax {($base:expr, $($cmps:expr),+ $(,)*) => {{let cmp_max = max!($($cmps),+);if $base < cmp_max {$base = cmp_max;true} else {false}}};}
macro_rules! min {
    ($a:expr $(,)*) => {{$a}};
    ($a:expr, $b:expr $(,)*) => {{if $a > $b {$b} else {$a}}};
    ($a:expr, $($rest:expr),+ $(,)*) => {{let b = min!($($rest),+);if $a > b {b} else {$a}}};
}
macro_rules! max {
    ($a:expr $(,)*) => {{$a}};
    ($a:expr, $b:expr $(,)*) => {{if $a > $b {$a} else {$b}}};
    ($a:expr, $($rest:expr),+ $(,)*) => {{let b = max!($($rest),+);if $a > b {$a} else {b}}};
}

//////////////////////////////////////

#[cfg(test)]
mod test {
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
}
