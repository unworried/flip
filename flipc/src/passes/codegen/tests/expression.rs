use flipvm::op::{Instruction, Literal12Bit, StackOp};
use flipvm::Register;

use crate::ast::visitor::Visitor;
use crate::ast::{Literal, LiteralKind};
use crate::{Ast, CodeGenerator};

#[test]
fn literal_int() {
    let ast = Ast::Literal(Literal {
        kind: LiteralKind::Int(123),
        span: Default::default(),
    });

    let mut gen = CodeGenerator::default();

    gen.visit_ast(&ast);

    let expected = vec![
        Instruction::Imm(Register::C, Literal12Bit { value: 123 }),
        Instruction::Stack(Register::C, Register::SP, StackOp::Push),
    ];

    assert_eq!(gen.instructions, expected);
}

#[test]
fn literal_char() {
    let ast = Ast::Literal(Literal {
        kind: LiteralKind::Char('h'),
        span: Default::default(),
    });

    let mut gen = CodeGenerator::default();

    gen.visit_ast(&ast);

    let expected = vec![
        Instruction::Imm(Register::C, Literal12Bit { value: 'h' as u16 }),
        Instruction::Stack(Register::C, Register::SP, StackOp::Push),
    ];

    assert_eq!(gen.instructions, expected);
}
