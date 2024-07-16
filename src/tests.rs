use crate::*;
use linkme::distributed_slice;

#[distributed_slice]
pub static TESTS: [fn()];

pub fn test_runner() {
    println!("Running {} tests", TESTS.len());
    for test in TESTS {
        test();
    }
}
