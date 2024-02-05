### RoadMap

#### Flow
=> Token/Byte Stream input
=> Lexer + Parser
=> Resolver
=> Type Checker
=> Desugaring
=> IR Lowering (HIR)
=> LLVM IR Codegen
=> Object file/Output

#### Frontend
- Input => Lexer => TokenStream
- TokenStream => Parser => AST
- AST => Resolver => Definitions Table
- Definitions Table: Resolver => Binary/Unary Evaluation
    - Issue Diagnostic Warning for binary operations not including vars.
    - 


#### Todo
- [ ] Test breaking rules
- [ ] Scope
- [ ] Variable visitor logic
- [ ] assignment after declaration logic
- [ ] Document funcs
- [ ] Grammer Doc
- [ ] Types
    - Should they be strict?
- [ ] functions


- [ ] Does ast need lifetimes for resolver?

- [ ] Include global long lived cache??

- [ ] Fix spans so they require less clones
