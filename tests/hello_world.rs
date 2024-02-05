use flipc::frontend;

use self::common::read_source_file;

mod common;

//#[test]
fn hello_world() {
    let src = read_source_file("helloworld.fl");
    match frontend::check(&src) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }
}
