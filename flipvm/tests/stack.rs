use flipvm::op::{Instruction::*, Literal12Bit, Nibble, StackOp};
use flipvm::Addressable;
use flipvm::Register::*;

use self::common::{init_machine, run, SIGHALT};

mod common;

#[test]
fn push() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(123).unwrap()),
        Stack(A, SP, StackOp::Push),
        Imm(A, Literal12Bit::new_checked(301).unwrap()),
        Stack(A, SP, StackOp::Push),
        Imm(A, Literal12Bit::new_checked(12).unwrap()),
        Stack(A, SP, StackOp::Push),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_mem_eq!(m, SP - 2, 12);
    assert_mem_eq!(m, SP - 4, 301);
    assert_mem_eq!(m, SP - 6, 123);
}

#[test]
fn pop() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(1).unwrap()),
        Stack(A, SP, StackOp::Push),
        Imm(A, Literal12Bit::new_checked(552).unwrap()),
        Stack(A, SP, StackOp::Push),
        Stack(B, SP, StackOp::Pop),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_reg_eq!(m, B, 552);
    assert_mem_eq!(m, SP - 2, 1);
}

#[test]
fn swap() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(231).unwrap()),
        Imm(B, Literal12Bit::new_checked(537).unwrap()),
        Stack(A, SP, StackOp::Push),
        Stack(B, SP, StackOp::Push),
        Stack(Zero, SP, StackOp::Swap),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_mem_eq!(m, SP - 2, 231);
    assert_mem_eq!(m, SP - 4, 537);
}

#[test]
fn peek() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(1).unwrap()),
        Stack(A, SP, StackOp::Push),
        Imm(A, Literal12Bit::new_checked(552).unwrap()),
        Stack(A, SP, StackOp::Push),
        Stack(B, SP, StackOp::Peek),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_reg_eq!(m, B, 552);
    assert_mem_eq!(m, SP - 2, 552);
}

#[test]
fn dup() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(98).unwrap()),
        Stack(A, SP, StackOp::Push),
        Stack(Zero, SP, StackOp::Dup),
        Stack(Zero, SP, StackOp::Dup),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_mem_eq!(m, SP - 2, 98);
    assert_mem_eq!(m, SP - 4, 98);
    assert_mem_eq!(m, SP - 6, 98);
}

#[test]
fn rotate() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(1).unwrap()),
        Stack(A, SP, StackOp::Push),
        Imm(A, Literal12Bit::new_checked(2).unwrap()),
        Stack(A, SP, StackOp::Push),
        Imm(A, Literal12Bit::new_checked(3).unwrap()),
        Stack(A, SP, StackOp::Push),
        Stack(Zero, SP, StackOp::Rotate),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_mem_eq!(m, SP - 2, 2);
    assert_mem_eq!(m, SP - 4, 1);
    assert_mem_eq!(m, SP - 6, 3);
}

#[test]
fn add() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(5).unwrap()),
        Stack(A, SP, StackOp::Push),
        Imm(A, Literal12Bit::new_checked(10).unwrap()),
        Stack(A, SP, StackOp::Push),
        Stack(Zero, SP, StackOp::Add),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_mem_eq!(m, SP - 2, 15);
}

#[test]
fn sub() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(5).unwrap()),
        Stack(A, SP, StackOp::Push),
        Imm(A, Literal12Bit::new_checked(20).unwrap()),
        Stack(A, SP, StackOp::Push),
        Stack(Zero, SP, StackOp::Sub),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_mem_eq!(m, SP - 2, 15);
}

#[test]
fn load_offset() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(105).unwrap()),
        Stack(A, SP, StackOp::Push),
        Imm(A, Literal12Bit::new_checked(210).unwrap()),
        Stack(A, SP, StackOp::Push),
        Imm(A, Literal12Bit::new_checked(315).unwrap()),
        Stack(A, SP, StackOp::Push),
        LoadStackOffset(C, SP, Nibble::new_checked(3).unwrap()),
        LoadStackOffset(B, SP, Nibble::new_checked(2).unwrap()),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_reg_eq!(m, C, 105);
    assert_reg_eq!(m, B, 210);
}
