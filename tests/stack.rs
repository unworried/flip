use flipvm::op::{Instruction::*, Nibble, StackOp};
use flipvm::Machine;
use flipvm::Register::*;

use self::common::{run, SIGHALT};

mod common;

#[test]
fn push() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, 123),
        Stack(A, SP, StackOp::Push),
        Imm(A, 301),
        Stack(A, SP, StackOp::Push),
        Imm(A, 12),
        Stack(A, SP, StackOp::Push),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_mem_eq!(vm, SP - 2, 12);
    assert_mem_eq!(vm, SP - 4, 301);
    assert_mem_eq!(vm, SP - 6, 123);
}

#[test]
fn pop() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, 1),
        Stack(A, SP, StackOp::Push),
        Imm(A, 552),
        Stack(A, SP, StackOp::Push),
        Stack(B, SP, StackOp::Pop),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, B, 552);
    assert_mem_eq!(vm, SP - 2, 1);
}

#[test]
fn swap() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, 231),
        Imm(B, 537),
        Stack(A, SP, StackOp::Push),
        Stack(B, SP, StackOp::Push),
        Stack(Zero, SP, StackOp::Swap),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_mem_eq!(vm, SP - 2, 231);
    assert_mem_eq!(vm, SP - 4, 537);
}

#[test]
fn peek() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, 1),
        Stack(A, SP, StackOp::Push),
        Imm(A, 552),
        Stack(A, SP, StackOp::Push),
        Stack(B, SP, StackOp::Peek),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, B, 552);
    assert_mem_eq!(vm, SP - 2, 552);
}

#[test]
fn dup() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, 98),
        Stack(A, SP, StackOp::Push),
        Stack(Zero, SP, StackOp::Dup),
        Stack(Zero, SP, StackOp::Dup),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_mem_eq!(vm, SP - 2, 98);
    assert_mem_eq!(vm, SP - 4, 98);
    assert_mem_eq!(vm, SP - 6, 98);
}

#[test]
fn rotate() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, 1),
        Stack(A, SP, StackOp::Push),
        Imm(A, 2),
        Stack(A, SP, StackOp::Push),
        Imm(A, 3),
        Stack(A, SP, StackOp::Push),
        Stack(Zero, SP, StackOp::Rotate),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_mem_eq!(vm, SP - 2, 2);
    assert_mem_eq!(vm, SP - 4, 1);
    assert_mem_eq!(vm, SP - 6, 3);
}

#[test]
fn add() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, 5),
        Stack(A, SP, StackOp::Push),
        Imm(A, 10),
        Stack(A, SP, StackOp::Push),
        Stack(Zero, SP, StackOp::Add),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_mem_eq!(vm, SP - 2, 15);
}

#[test]
fn sub() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, 5),
        Stack(A, SP, StackOp::Push),
        Imm(A, 20),
        Stack(A, SP, StackOp::Push),
        Stack(Zero, SP, StackOp::Sub),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_mem_eq!(vm, SP - 2, 15);
}

#[test]
fn load_offset() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, 105),
        Stack(A, SP, StackOp::Push),
        Imm(A, 210),
        Stack(A, SP, StackOp::Push),
        Imm(A, 315),
        Stack(A, SP, StackOp::Push),
        LoadStackOffset(C, SP, Nibble::new(3)),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, C, 105);
}
