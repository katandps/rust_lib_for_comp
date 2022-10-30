//! # 時間計測
use crate::prelude::*;

#[snippet(name = "time-keeper")]
pub fn get_time() -> f64 {
    static mut STIME: Option<f64> = None;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME.is_none() {
            STIME = Some(ms);
        }
        STIME.map(|k| ms - k).unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(0.0, get_time());
    let mut a = 1;
    for _ in 0..10000000 {
        a *= 2;
        a %= 5;
    }
    assert_eq!(a, 1);
    assert!(get_time() > 0.0);
}
