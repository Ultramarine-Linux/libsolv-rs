use std::io::Result;
use std::path::Path;
use ::solver::Solver;
use libsolv_sys::Solver as _Solver;
use libsolv_sys::solv_free;
use ::pool::PoolContext;
use ::queue::Queue;
use std::ptr;
use std::ffi::CString;
use libc::{c_char, c_int, FILE};
use ownership::SolvTake;
use libc;



pub fn read<P: AsRef<Path>>(pool: &PoolContext, path: P, job: &mut Queue) -> Result<(Solver, CString, c_int)> {
    use libsolv_sys::testcase_read;

    let fp: *mut FILE = ptr::null_mut();
    let mut resultp: *mut c_char = ptr::null_mut();
    let mut resultflagsp: c_int = 0;
    // TODO: fix unwraps
    let testcase = CString::new(path.as_ref().to_str().unwrap()).unwrap();
    let solver: *mut _Solver = {
        let borrow = pool.borrow_mut();
        unsafe {testcase_read(borrow._p, fp, testcase.as_ptr(), &mut job._q, &mut resultp, &mut resultflagsp)}
    };

    let resultpString = unsafe {CString::solv_take_mut(&mut resultp)}.unwrap();

    //TODO: We left off here. Use path, not testcase. Check solver result
    Ok((Solver::new_with_solver(pool.clone_context(), solver), resultpString, resultflagsp))
}

pub fn solverresult(solver: &mut Solver, resultflags: c_int) -> Option<CString> {
    use libsolv_sys::testcase_solverresult;

    unsafe {
        let _ = solver.ctx.borrow_mut();
        let mut resultstr = testcase_solverresult(solver._s, resultflags);
        CString::solv_take_mut(&mut resultstr)
    }
}
