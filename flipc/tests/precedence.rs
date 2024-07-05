use flipc::frontend;

use self::common::read_source_file;

mod common;

#[test]
fn precedence() {
    let src = read_source_file("precedence.fl");
    match frontend::check(&src) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }
}
