// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;






pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
        foreign_links {
            Nul(::std::ffi::NulError);
        }
    }
}

pub mod checksum;
pub mod pool;
pub mod queue;
pub mod repo;
pub mod solver;
pub mod sys;
pub mod transaction;

mod ownership;

pub use libsolv_sys::{solv_knownid, Id};

#[cfg(feature = "ext")]
pub mod ext;

#[cfg(test)]
mod tests {
    use std::path::{Path};
    use crate::ext::testcase;
    use crate::queue::Queue;
    use crate::pool::{PoolContext};

    #[test]
    fn test_solv() {
        // print current path
        println!("{:?}", std::env::current_dir().unwrap());
        let path = Path::new("libsolv-source/test/testcases/choose/default.t");
        let mut job = Queue::new();
        let pool = PoolContext::new();
        if let Ok((mut solver, _result, resultflags)) = testcase::read(&pool, path, &mut job) {
            solver.solve(&mut job);
            let myresult = testcase::solverresult(&mut solver, resultflags).unwrap();
            println!("{:?}", myresult);
        }
    }
}
