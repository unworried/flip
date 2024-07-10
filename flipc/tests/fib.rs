use flipc::frontend;

use self::common::read_source_file;

mod common;

#[test]
fn fib() {
    let src = read_source_file("fib.fl");
    match frontend::check(&src) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }
}
