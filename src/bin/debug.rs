use flipc::frontend;

fn main() {
    /*let line = r#"let x = 4;
        while x == 1 {
        if x == 4 {
            x = 5;
            let y = 5;
        };
        let z = 6* "";
    };
    let xyz = "abc";
    "#;*/
    // Fix so not lit strings allowed in binary operations

    //let line = r#"let x = 4; x = 5; let y = 6; x = y;"#;
    //let line = r#"djaindahbdjhbajhb dhjbajh bdjhbah bdhjab hjbdhjbah jbdjhab dhjbajh bdhabh jdbhja bdjhbajhdbajhdbjhdb"#;
    //let line = r#"let x = 45+41*(4+3); x = 5 + 2; let y = -x;"#;
    //let line = r#"let x = 4; let y=x; x = y; x = 7; y = x;"#;
    let line = r#"let x = 1; x = x + 1;"#;
    //let line = "while \"TMP\" { \nlet x = \"hello, world!\"; \nlet y = \"hello, world 2!\"; \nlet z = \"hello, world 3!\"; \n };";
    match frontend::check(line) {
        Ok(_) => println!("No errors found"),
        Err(e) => println!("{}", e),
    }
}
