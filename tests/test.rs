use flipvm::op::{Instruction::*, Literal12Bit, Nibble, TestOp};
use flipvm::Flag;
use flipvm::Register::*;

use self::common::{init_vm, run, SIGHALT};

mod common;
macro_rules! test_set {
    ($vm:ident, $($prog:expr),+) => {
        let program = &[$($prog),*, System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap())];
        run(&mut $vm, program).unwrap();
        assert_flag_set!($vm, Flag::Compare);
    }
}

macro_rules! test_unset {
    ($vm:ident, $($prog:expr),+) => {
        let program = &[$($prog),*, System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap())];
        run(&mut $vm, program).unwrap();
        assert_flag_unset!($vm, Flag::Compare);
    }
}

#[test]
fn eq() {
    let mut vm = init_vm(1024 * 4);
    test_unset!(
        vm,
        Imm(A, Literal12Bit::new_checked(123).unwrap()),
        Imm(B, Literal12Bit::new_checked(567).unwrap()),
        Test(A, B, TestOp::Eq)
    );
    vm.reset();
    test_set!(
        vm,
        Imm(A, Literal12Bit::new_checked(444).unwrap()),
        Imm(B, Literal12Bit::new_checked(444).unwrap()),
        Test(A, B, TestOp::Eq)
    );
}

#[test]
fn neq() {
    let mut vm = init_vm(1024 * 4);
    test_set!(
        vm,
        Imm(A, Literal12Bit::new_checked(123).unwrap()),
        Imm(B, Literal12Bit::new_checked(567).unwrap()),
        Test(A, B, TestOp::Neq)
    );
    vm.reset();
    test_unset!(
        vm,
        Imm(A, Literal12Bit::new_checked(444).unwrap()),
        Imm(B, Literal12Bit::new_checked(444).unwrap()),
        Test(A, B, TestOp::Neq)
    );
}

#[test]
fn lt() {
    let mut vm = init_vm(1024 * 4);

    test_set!(
        vm,
        Imm(A, Literal12Bit::new_checked(44).unwrap()),
        Imm(B, Literal12Bit::new_checked(55).unwrap()),
        Test(A, B, TestOp::Lt)
    );
    vm.reset();
    test_unset!(
        vm,
        Imm(A, Literal12Bit::new_checked(88).unwrap()),
        Imm(B, Literal12Bit::new_checked(44).unwrap()),
        Test(A, B, TestOp::Lt)
    );
    vm.reset();
    test_set!(
        vm,
        Imm(A, Literal12Bit::new_checked(55).unwrap()),
        Imm(B, Literal12Bit::new_checked(55).unwrap()),
        Test(A, B, TestOp::Lte)
    );
    vm.reset();
    test_unset!(
        vm,
        Imm(A, Literal12Bit::new_checked(88).unwrap()),
        Imm(B, Literal12Bit::new_checked(44).unwrap()),
        Test(A, B, TestOp::Lte)
    );
}

#[test]
fn gt() {
    let mut vm = init_vm(1024 * 4);

    test_unset!(
        vm,
        Imm(A, Literal12Bit::new_checked(44).unwrap()),
        Imm(B, Literal12Bit::new_checked(55).unwrap()),
        Test(A, B, TestOp::Gt)
    );
    vm.reset();
    test_set!(
        vm,
        Imm(A, Literal12Bit::new_checked(88).unwrap()),
        Imm(B, Literal12Bit::new_checked(44).unwrap()),
        Test(A, B, TestOp::Gt)
    );
    vm.reset();
    test_set!(
        vm,
        Imm(A, Literal12Bit::new_checked(55).unwrap()),
        Imm(B, Literal12Bit::new_checked(55).unwrap()),
        Test(A, B, TestOp::Gte)
    );
    vm.reset();
    test_unset!(
        vm,
        Imm(A, Literal12Bit::new_checked(44).unwrap()),
        Imm(B, Literal12Bit::new_checked(88).unwrap()),
        Test(A, B, TestOp::Gte)
    );
}

#[test]
fn both_zero() {
    let mut vm = init_vm(1024 * 4);
    test_unset!(
        vm,
        Imm(A, Literal12Bit::new_checked(44).unwrap()),
        Test(A, B, TestOp::BothZero)
    );
    vm.reset();
    test_set!(vm, Test(A, B, TestOp::BothZero));
    vm.reset();
    test_set!(vm, Test(Zero, Zero, TestOp::BothZero));
}

#[test]
fn either_nonzero() {
    let mut vm = init_vm(1024 * 4);
    test_set!(
        vm,
        Imm(A, Literal12Bit::new_checked(44).unwrap()),
        Test(A, B, TestOp::EitherNonZero)
    );
    vm.reset();
    test_unset!(vm, Test(A, B, TestOp::EitherNonZero));
    vm.reset();
    test_unset!(vm, Test(Zero, Zero, TestOp::EitherNonZero));
}

#[test]
fn both_nonzero() {
    let mut vm = init_vm(1024 * 4);
    test_unset!(
        vm,
        Imm(A, Literal12Bit::new_checked(44).unwrap()),
        Test(A, B, TestOp::BothNonZero)
    );
    vm.reset();
    test_unset!(vm, Test(Zero, Zero, TestOp::BothNonZero));
    vm.reset();
    test_set!(
        vm,
        Imm(A, Literal12Bit::new_checked(1).unwrap()),
        Imm(B, Literal12Bit::new_checked(2).unwrap()),
        Test(A, B, TestOp::BothNonZero)
    );
}
