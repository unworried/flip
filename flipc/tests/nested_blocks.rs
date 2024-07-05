use flipc::frontend;

use self::common::read_source_file;

mod common;

#[test]
fn nested_blocks() {
    let src = read_source_file("nestedblocks.fl");
    match frontend::check(&src) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }
}
