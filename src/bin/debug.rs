use flipc::frontend;

fn main() {
    let line = r#"let x = 4; let x = 4;
    while x == 1 {
        if x == 4 {
            x = 5;
            let y = 5;
        };
        let z = 6;
    };
    "#;

    match frontend::check(line) {
        Ok(_) => println!("No errors found"),
        Err(e) => println!("{}", e),
    }
}
