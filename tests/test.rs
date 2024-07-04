use flipvm::op::{Instruction::*, Nibble, TestOp};
use flipvm::Flag;
use flipvm::Machine;
use flipvm::Register::*;

use self::common::{run, SIGHALT};

mod common;
macro_rules! test_set {
    ($vm:ident, $($prog:expr),+) => {
        let program = &[$($prog),*, System(Zero, Zero, Nibble::new(SIGHALT))];
        run(&mut $vm, program).unwrap();
        assert_flag_set!($vm, Flag::Compare);
    }
}

macro_rules! test_unset {
    ($vm:ident, $($prog:expr),+) => {
        let program = &[$($prog),*, System(Zero, Zero, Nibble::new(SIGHALT))];
        run(&mut $vm, program).unwrap();
        assert_flag_unset!($vm, Flag::Compare);
    }
}

#[test]
fn eq() {
    let mut vm = Machine::new(1024 * 4);
    test_unset!(vm, Imm(A, 123), Imm(B, 567), Test(A, B, TestOp::Eq));
    vm.reset();
    test_set!(vm, Imm(A, 444), Imm(B, 444), Test(A, B, TestOp::Eq));
}

#[test]
fn neq() {
    let mut vm = Machine::new(1024 * 4);
    test_set!(vm, Imm(A, 123), Imm(B, 567), Test(A, B, TestOp::Neq));
    vm.reset();
    test_unset!(vm, Imm(A, 444), Imm(B, 444), Test(A, B, TestOp::Neq));
}

#[test]
fn lt() {
    let mut vm = Machine::new(1024 * 4);
    test_set!(vm, Imm(A, 44), Imm(B, 55), Test(A, B, TestOp::Lt));
    vm.reset();
    test_unset!(vm, Imm(A, 88), Imm(B, 44), Test(A, B, TestOp::Lt));
    vm.reset();
    test_set!(vm, Imm(A, 55), Imm(B, 55), Test(A, B, TestOp::Lte));
    vm.reset();
    test_unset!(vm, Imm(A, 88), Imm(B, 44), Test(A, B, TestOp::Lte));
}

#[test]
fn gt() {
    let mut vm = Machine::new(1024 * 4);
    test_unset!(vm, Imm(A, 44), Imm(B, 55), Test(A, B, TestOp::Gt));
    vm.reset();
    test_set!(vm, Imm(A, 88), Imm(B, 44), Test(A, B, TestOp::Gt));
    vm.reset();
    test_set!(vm, Imm(A, 55), Imm(B, 55), Test(A, B, TestOp::Gte));
    vm.reset();
    test_unset!(vm, Imm(A, 44), Imm(B, 88), Test(A, B, TestOp::Gte));
}

#[test]
fn both_zero() {
    let mut vm = Machine::new(1024 * 4);
    test_unset!(vm, Imm(A, 44), Test(A, B, TestOp::BothZero));
    vm.reset();
    test_set!(vm, Test(A, B, TestOp::BothZero));
    vm.reset();
    test_set!(vm, Test(Zero, Zero, TestOp::BothZero));
}

#[test]
fn either_nonzero() {
    let mut vm = Machine::new(1024 * 4);
    test_set!(vm, Imm(A, 44), Test(A, B, TestOp::EitherNonZero));
    vm.reset();
    test_unset!(vm, Test(A, B, TestOp::EitherNonZero));
    vm.reset();
    test_unset!(vm, Test(Zero, Zero, TestOp::EitherNonZero));
}

#[test]
fn both_nonzero() {
    let mut vm = Machine::new(1024 * 4);
    test_unset!(vm, Imm(A, 44), Test(A, B, TestOp::BothNonZero));
    vm.reset();
    test_unset!(vm, Test(Zero, Zero, TestOp::BothNonZero));
    vm.reset();
    test_set!(vm, Imm(A, 1), Imm(B, 2), Test(A, B, TestOp::BothNonZero));
}
