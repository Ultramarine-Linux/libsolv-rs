



use std::path::{Path};
use libsolv::ext::testcase;
use libsolv::queue::Queue;
use libsolv::pool::{PoolContext};


fn main() {
    let path = Path::new("../libsolv/test/testcases/choose/default.t");
    let mut job = Queue::new();
    let pool = PoolContext::new();
    if let Ok((mut solver, _result, resultflags)) = testcase::read(&pool, path, &mut job) {
        solver.solve(&mut job);
        let myresult = testcase::solverresult(&mut solver, resultflags).unwrap();
        println!("{:?}", myresult);
    }
}