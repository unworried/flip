use flipvm::op::{Instruction::*, Literal10Bit, Nibble, TestOp};
use flipvm::Machine;
use flipvm::Register::*;

use self::common::{run, SIGHALT};

mod common;

#[test]
fn jump() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(PC, 10),
        Invalid(0),
        Invalid(0),
        Invalid(0),
        Invalid(0),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, PC, 12);
}

#[test]
fn jump_offset() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Add(Zero, Zero, Zero),
        Add(Zero, Zero, Zero),
        Add(Zero, Zero, Zero),
        JumpOffset(Literal10Bit::new(10)),
        Invalid(0),
        Invalid(0),
        Invalid(0),
        Invalid(0),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, PC, 18);
}

#[test]
fn branch() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, 12),
        Imm(B, 13),
        Test(A, B, TestOp::Neq),
        AddIf(PC, PC, Nibble::new(0x4)),
        Invalid(0),
        Invalid(0),
        Invalid(0),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
}

#[test]
fn branch_without_test() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, 12),
        Imm(B, 13),
        Test(A, B, TestOp::Neq),
        AddIf(PC, PC, Nibble::new(0x3)),
        Invalid(0),
        Invalid(0),
        AddIf(PC, PC, Nibble::new(0xf)),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
}

#[test]
fn jump_and_link_set() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(B, 4),
        SetAndSave(PC, B, C),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, C, 2);
}

#[test]
fn jump_and_link_add() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, 8),
        AddAndSave(PC, A, B),
        Invalid(0),
        Invalid(0),
        Invalid(0),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, B, 2);
}
