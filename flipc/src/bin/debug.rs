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

    // TODO FIX THIS NO ERROR EMITED BY COMPILER
    // let line = r#"let x = 7; x = 1 let y = x - 2;"#;

    //let line = r#"let x = 4; x = 5; let y = 6; x = y;"#;
    //let line = r#"djaindahbdjhbajhb dhjbajh bdjhbah bdhjab hjbdhjbah jbdjhab dhjbajh bdhabh jdbhja bdjhbajhdbajhdbjhdb"#;
    //let line = r#"let x = 45+41*(4+3); x = 5 + 2; let y = -x;"#;
    //let line = r#"let x = 4; let y=x; x = y; x = 7; y = x;"#;
    //let line = r#"let x = 1; x = x + 1;"#;
    //let line = "while \"TMP\" { \nlet x = \"hello, world!\"; \nlet y = \"hello, world 2!\"; \nlet z = \"hello, world 3!\"; \n };";

    //let line = r#"main() { if 1 <= 1 { if 1 <= 1 { let x = 1; }; }; }"#;
    let line = r#"void main() {
        let x = 'a';
        let y = 10;
        return fib(y);
    }
    
    int fib(n) {
        if n == 0 {
            return 0;
        };

        if n == 1 {
            return 1;
        };

        let t1 = fib(n - 1);
        let t2 = fib(n - 2);
        return t1 + t2;
    }"#;
    match frontend::check(line) {
        Ok(_) => eprintln!("No errors found"),
        Err(e) => eprintln!("{}", e),
    }
}
