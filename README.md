# Flip
Flip is a custom compiler and virtual machine designed to parse, compile and run a simple programming language. The language is designed to be simple and easy to understand, compiled into bytecode using a 16-bit instruction set.

## Features
- Lexical analysis, tokenization of the source code
- Combinator parser to generate an abstract syntax tree (AST)
- Multiple AST passes to facilitate compile time checks prior to codegen
- Codegen to generate bytecode from the AST using a 16-bit instruction set to be run on a toy virtual machine
- Virtual machine to run the bytecode
- REPL to run code interactively

## Project Structure
- `flipc`: The compiler
- `flipvm`: The virtual machine

## Supported Language Features
- Arithmetic operations: `+`, `-`, `*`, `/`
- Comparison operations: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Variable declarations and assignments
- If statements
- While loops
- Nested blocks/scopes
- Strict scoping (no shadowing)

## Building and Running
TBA

## Examples
```bash
let foo = 1;
while 1 >= 1 { 
    # let foo = 400;
    if foo < 401 {
        while 1 == 1 {
            let bap = foo;
        };
    };
};

let baz = 0;
# let baz = 1;
# let bar = baz * 3 + 2;
if foo > 0 {
    let bap = "foo is positive!";
    baz = bap;
};
```

## Future Improvements
- Implement function declarations and calls
- Add support for more complex data types
- Implement optimizations and code generation phase
