use flipvm::op::Instruction::*;
use flipvm::op::{Literal10Bit, Literal12Bit, Nibble, TestOp};
use flipvm::Register::*;

use self::common::{init_machine, run, SIGHALT};

mod common;

#[test]
fn jump() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(PC, Literal12Bit::new_checked(10).unwrap()),
        Invalid,
        Invalid,
        Invalid,
        Invalid,
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_reg_eq!(m, PC, 12);
}

#[test]
fn jump_offset() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Add(Zero, Zero, Zero),
        Add(Zero, Zero, Zero),
        Add(Zero, Zero, Zero),
        JumpOffset(Literal10Bit::new_checked(10).unwrap()),
        Invalid,
        Invalid,
        Invalid,
        Invalid,
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_reg_eq!(m, PC, 18);
}

#[test]
fn branch() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(12).unwrap()),
        Imm(B, Literal12Bit::new_checked(13).unwrap()),
        Test(A, B, TestOp::Neq),
        AddIf(PC, PC, Nibble::new_checked(0x4).unwrap()),
        Invalid,
        Invalid,
        Invalid,
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
}

#[test]
fn branch_without_test() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(12).unwrap()),
        Imm(B, Literal12Bit::new_checked(13).unwrap()),
        Test(A, B, TestOp::Neq),
        AddIf(PC, PC, Nibble::new_checked(0x3).unwrap()),
        Invalid,
        Invalid,
        AddIf(PC, PC, Nibble::new_checked(0xf).unwrap()),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
}

#[test]
fn jump_and_link_set() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(B, Literal12Bit::new_checked(4).unwrap()),
        SetAndSave(PC, B, C),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_reg_eq!(m, C, 2);
}

#[test]
fn jump_and_link_add() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(8).unwrap()),
        AddAndSave(PC, A, B),
        Invalid,
        Invalid,
        Invalid,
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_reg_eq!(m, B, 2);
}
