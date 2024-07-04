use flipvm::op::Nibble;
use flipvm::op::{Instruction::*, Literal7Bit};
use flipvm::{Machine, Register::*};

use self::common::{run, SIGHALT};

mod common;

#[test]
fn add() {
    let mut vm = Machine::new(1024 * 4);
    vm.reset();
    let program = vec![
        Imm(A, 11),
        Imm(B, 15),
        Add(A, B, C),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, C, 26);
}

#[test]
fn sub() {
    let mut vm = Machine::new(1024 * 4);
    vm.reset();
    let program = vec![
        Imm(A, 20),
        Imm(B, 15),
        Sub(A, B, C),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, C, 5);
}

#[test]
fn sub_overflow() {
    let mut vm = Machine::new(1024 * 4);
    vm.reset();
    let program = vec![
        Imm(A, 1),
        Imm(B, 57),
        Sub(A, B, C),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, C, u16::MAX - 55);
}

#[test]
fn add_imm() {
    let mut vm = Machine::new(1024 * 4);
    vm.reset();
    let program = vec![
        Imm(A, 11),
        AddImm(A, Literal7Bit::new(4)),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, A, 15);
}

#[test]
fn shift_left() {
    let mut vm = Machine::new(1024 * 4);
    vm.reset();
    let program = vec![
        Imm(C, 0xff),
        ShiftLeft(C, B, Nibble::new(4)),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, B, 0xff0);
}

#[test]
fn shift_right_logical() {
    let mut vm = Machine::new(1024 * 4);
    vm.reset();
    let program = vec![
        Imm(B, 0x8fc),
        ShiftLeft(B, B, Nibble::new(4)),
        AddImm(B, Literal7Bit::new(0x7)),
        // 0x8fc7
        ShiftRightLogical(B, A, Nibble::new(3)),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, A, 0x11f8);
}

#[test]
fn shift_right_arithmetic() {
    let mut vm = Machine::new(1024 * 4);
    vm.reset();
    let program = vec![
        Imm(A, 0xff0),
        ShiftLeft(A, A, Nibble::new(4)),
        AddImm(A, Literal7Bit::new(0x70)),
        // 0xff70
        ShiftRightArithmetic(A, C, Nibble::new(2)),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, C, 0xffdc);
}
