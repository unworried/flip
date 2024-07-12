use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;

use flipvm::op::{Instruction, Literal12Bit, Literal7Bit, Nibble, StackOp, TestOp};
use flipvm::Register::*;

use crate::diagnostics::DiagnosticBag;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::passes::nameresolver::NameResolver;
use crate::passes::symbol_table::SymbolTableBuilder;
use crate::passes::Pass;

use super::CodeGenerator;

mod expression;

impl<'a> Default for CodeGenerator<'a> {
    fn default() -> Self {
        Self {
            inital_offset: 0,
            current_offset: 0,
            instructions: Vec::new(),
            symbol_table: RefCell::new(Default::default()),
            scope_idx: 0,
            labels: HashMap::new(),
            unlinked_references: Vec::new(),
            _phantom: PhantomData::<&'a ()>,
        }
    }
}

#[test]
fn simple_program() {
    let input = r#"
main() {
    let x = 1;
    if x == 2 {
        x = 3;
    };
}
        "#;

    let diagnostics = DiagnosticBag::new();

    // Fix to make lexer take src
    let mut lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(&mut lexer, diagnostics.clone());

    let root = parser.parse();

    let (st, mut ft) = SymbolTableBuilder::run((&root, diagnostics.clone()));
    let st = NameResolver::run((&root, st, &mut ft, diagnostics.clone()));

    let actual = CodeGenerator::run((&root, st, 0x0));

    let expected = vec![
        Instruction::Imm(SP, Literal12Bit { value: 1023 }),
        Instruction::ShiftLeft(SP, SP, Nibble { value: 4 }),
        Instruction::Stack(BP, SP, StackOp::Push),
        Instruction::Stack(PC, SP, StackOp::Push),
        Instruction::Add(SP, Zero, BP),
        Instruction::Imm(PC, Literal12Bit { value: 16 }),
        Instruction::Imm(C, Literal12Bit { value: 240 }),
        Instruction::System(C, Zero, Nibble { value: 0 }),
        Instruction::AddImm(SP, Literal7Bit { value: 2 }),
        Instruction::Imm(C, Literal12Bit { value: 1 }),
        Instruction::Stack(C, SP, StackOp::Push),
        Instruction::Stack(C, SP, StackOp::Pop),
        Instruction::Add(BP, Zero, B),
        Instruction::AddImm(B, Literal7Bit { value: 0 }),
        Instruction::StoreWord(B, Zero, C),
        Instruction::Imm(C, Literal12Bit { value: 2 }),
        Instruction::Stack(C, SP, StackOp::Push),
        Instruction::Add(BP, Zero, C),
        Instruction::AddImm(C, Literal7Bit { value: 0 }),
        Instruction::LoadWord(C, C, Zero),
        Instruction::Stack(C, SP, StackOp::Push),
        Instruction::Stack(B, SP, StackOp::Pop),
        Instruction::Stack(C, SP, StackOp::Pop),
        Instruction::Test(B, C, TestOp::Eq),
        Instruction::Add(Zero, Zero, C),
        Instruction::AddIf(C, Zero, Nibble { value: 1 }),
        Instruction::Stack(C, SP, StackOp::Push),
        Instruction::Stack(C, SP, StackOp::Pop),
        Instruction::Test(C, Zero, TestOp::BothZero),
        Instruction::AddIf(PC, PC, Nibble { value: 2 }),
        Instruction::Imm(PC, Literal12Bit { value: 64 }),
        Instruction::Imm(PC, Literal12Bit { value: 78 }),
        Instruction::Imm(C, Literal12Bit { value: 3 }),
        Instruction::Stack(C, SP, StackOp::Push),
        Instruction::Stack(C, SP, StackOp::Pop),
        Instruction::Add(BP, Zero, B),
        Instruction::AddImm(B, Literal7Bit { value: 0 }),
        Instruction::StoreWord(B, Zero, C),
        Instruction::Imm(PC, Literal12Bit { value: 78 }),
        Instruction::LoadStackOffset(C, BP, Nibble { value: 1 }),
        Instruction::Add(BP, Zero, SP),
        Instruction::AddImmSigned(SP, Literal7Bit { value: 126 }),
        Instruction::LoadStackOffset(BP, BP, Nibble { value: 2 }),
        Instruction::AddImm(C, Literal7Bit { value: 6 }),
        Instruction::Add(C, Zero, PC),
    ];

    assert_eq!(actual, expected);
}
