use flipvm::op::{Instruction::*, Literal7Bit};
use flipvm::op::{Literal12Bit, Nibble};
use flipvm::{Machine, Register::*};

use self::common::{run, SIGHALT};

mod common;

#[test]
fn add() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(11).unwrap()),
        Imm(B, Literal12Bit::new_checked(15).unwrap()),
        Add(A, B, C),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, C, 26);
}

#[test]
fn sub() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(20).unwrap()),
        Imm(B, Literal12Bit::new_checked(15).unwrap()),
        Sub(A, B, C),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, C, 5);
}

#[test]
fn sub_overflow() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(1).unwrap()),
        Imm(B, Literal12Bit::new_checked(57).unwrap()),
        Sub(A, B, C),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, C, u16::MAX - 55);
}

#[test]
fn add_imm() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(11).unwrap()),
        AddImm(A, Literal7Bit::new(4)),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, A, 15);
}

#[test]
fn add_imm_signed() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(21).unwrap()),
        AddImmSigned(A, Literal7Bit::from_signed(-4).unwrap()),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, A, 17);
}

#[test]
fn add_imm_signed_to_zero() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(C, Literal12Bit::new_checked(21).unwrap()),
        AddImmSigned(C, Literal7Bit::from_signed(-21).unwrap()),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, C, 0);
}

#[test]
fn shift_left() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(C, Literal12Bit::new_checked(0xff).unwrap()),
        ShiftLeft(C, B, Nibble::new(4)),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, B, 0xff0);
}

#[test]
fn shift_right_logical() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(B, Literal12Bit::new_checked(0x8fc).unwrap()),
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
    let program = vec![
        Imm(A, Literal12Bit::new_checked(0xff0).unwrap()),
        ShiftLeft(A, A, Nibble::new(4)),
        AddImm(A, Literal7Bit::new(0x70)),
        // 0xff70
        ShiftRightArithmetic(A, C, Nibble::new(2)),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, C, 0xffdc);
}
