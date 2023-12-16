//! # Area
use rust_lib_for_comp::geometry::convex_hull::Polygon;
use rust_lib_for_comp::util::io_util::*;

#[verify::aizu_online_judge("CGL_3_A", eps = "1e-6")]
pub fn cgl_3_a(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::default().add(read);
    let n = reader.v::<usize>();
    let xy = reader.vec2::<f64, f64>(n);
    let polygon = Polygon::from(&xy[..]);
    writeln!(write, "{}", polygon.area()).ok();
    write.flush().ok();
}

#[test]
fn test() {
    use rust_lib_for_comp::template::test_helper;
    test_helper(
        "3
    0 0
    2 2
    -1 1",
        "2.0",
    );
    test_helper(
        "4
    0 0
    1 1
    1 2
    0 2",
        "1.5",
    )
}
