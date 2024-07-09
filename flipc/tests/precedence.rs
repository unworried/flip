use flipc::frontend;

use self::common::read_source_file;

mod common;

#[ignore]
#[test]
fn precedence() {
    let src = read_source_file("precedence.fl");
    match frontend::check(&src) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }
}
