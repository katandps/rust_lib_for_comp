use rust_lib_for_comp::data_structure::persistent_queue::PersistentQueue;
use rust_lib_for_comp::util::io_util::*;
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct PersistentQueueSolver;
impl verify::Solver for PersistentQueueSolver {
    const PROBLEM_ID: &'static str = "persistent_queue";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let mut q = PersistentQueue::default();
        let mut timeline = Vec::new();
        for _ in 0..reader.v::<usize>() {
            let p = if 0 == reader.v::<usize>() {
                let (t, x) = reader.v2::<i64, usize>();
                if t == -1 {
                    q.push_rear(x, 0)
                } else {
                    q.push_rear(x, timeline[t as usize])
                }
            } else {
                let t: usize = reader.v::<usize>();
                let Some((v, p)) = q.pop_front(timeline[t]) else {
                    unreachable!()
                };
                writeln!(write, "{v}").ok();
                p
            };
            timeline.push(p);
        }
    }
}
#[test]
fn test() {
    PersistentQueueSolver::assert(
        "6
        0 -1 6
        0 0 7
        1 0
        0 -1 8
        1 3
        1 1",
        "6
        8
        6",
    );
}
