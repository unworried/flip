use flipvm::op::Instruction::*;
use flipvm::op::{Literal12Bit, Nibble, TestOp};
use flipvm::Flag;
use flipvm::Register::*;

use self::common::{init_machine, run, SIGHALT};

mod common;
macro_rules! test_set {
    ($m:ident, $($prog:expr),+) => {
        let program = &[$($prog),*, System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap())];
        run(&mut $m, program).unwrap();
        assert_flag_set!($m, Flag::Compare);
    }
}

macro_rules! test_unset {
    ($m:ident, $($prog:expr),+) => {
        let program = &[$($prog),*, System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap())];
        run(&mut $m, program).unwrap();
        assert_flag_unset!($m, Flag::Compare);
    }
}

#[test]
fn eq() {
    let mut m = init_machine(1024 * 4);
    test_unset!(
        m,
        Imm(A, Literal12Bit::new_checked(123).unwrap()),
        Imm(B, Literal12Bit::new_checked(567).unwrap()),
        Test(A, B, TestOp::Eq)
    );
    m.reset();
    test_set!(
        m,
        Imm(A, Literal12Bit::new_checked(444).unwrap()),
        Imm(B, Literal12Bit::new_checked(444).unwrap()),
        Test(A, B, TestOp::Eq)
    );
}

#[test]
fn neq() {
    let mut m = init_machine(1024 * 4);
    test_set!(
        m,
        Imm(A, Literal12Bit::new_checked(123).unwrap()),
        Imm(B, Literal12Bit::new_checked(567).unwrap()),
        Test(A, B, TestOp::Neq)
    );
    m.reset();
    test_unset!(
        m,
        Imm(A, Literal12Bit::new_checked(444).unwrap()),
        Imm(B, Literal12Bit::new_checked(444).unwrap()),
        Test(A, B, TestOp::Neq)
    );
}

#[test]
fn lt() {
    let mut m = init_machine(1024 * 4);

    test_set!(
        m,
        Imm(A, Literal12Bit::new_checked(44).unwrap()),
        Imm(B, Literal12Bit::new_checked(55).unwrap()),
        Test(A, B, TestOp::Lt)
    );
    m.reset();
    test_unset!(
        m,
        Imm(A, Literal12Bit::new_checked(88).unwrap()),
        Imm(B, Literal12Bit::new_checked(44).unwrap()),
        Test(A, B, TestOp::Lt)
    );
    m.reset();
    test_set!(
        m,
        Imm(A, Literal12Bit::new_checked(55).unwrap()),
        Imm(B, Literal12Bit::new_checked(55).unwrap()),
        Test(A, B, TestOp::Lte)
    );
    m.reset();
    test_unset!(
        m,
        Imm(A, Literal12Bit::new_checked(88).unwrap()),
        Imm(B, Literal12Bit::new_checked(44).unwrap()),
        Test(A, B, TestOp::Lte)
    );
}

#[test]
fn gt() {
    let mut m = init_machine(1024 * 4);

    test_unset!(
        m,
        Imm(A, Literal12Bit::new_checked(44).unwrap()),
        Imm(B, Literal12Bit::new_checked(55).unwrap()),
        Test(A, B, TestOp::Gt)
    );
    m.reset();
    test_set!(
        m,
        Imm(A, Literal12Bit::new_checked(88).unwrap()),
        Imm(B, Literal12Bit::new_checked(44).unwrap()),
        Test(A, B, TestOp::Gt)
    );
    m.reset();
    test_set!(
        m,
        Imm(A, Literal12Bit::new_checked(55).unwrap()),
        Imm(B, Literal12Bit::new_checked(55).unwrap()),
        Test(A, B, TestOp::Gte)
    );
    m.reset();
    test_unset!(
        m,
        Imm(A, Literal12Bit::new_checked(44).unwrap()),
        Imm(B, Literal12Bit::new_checked(88).unwrap()),
        Test(A, B, TestOp::Gte)
    );
}

#[test]
fn both_zero() {
    let mut m = init_machine(1024 * 4);
    test_unset!(
        m,
        Imm(A, Literal12Bit::new_checked(44).unwrap()),
        Test(A, B, TestOp::BothZero)
    );
    m.reset();
    test_set!(m, Test(A, B, TestOp::BothZero));
    m.reset();
    test_set!(m, Test(Zero, Zero, TestOp::BothZero));
}

#[test]
fn either_nonzero() {
    let mut m = init_machine(1024 * 4);
    test_set!(
        m,
        Imm(A, Literal12Bit::new_checked(44).unwrap()),
        Test(A, B, TestOp::EitherNonZero)
    );
    m.reset();
    test_unset!(m, Test(A, B, TestOp::EitherNonZero));
    m.reset();
    test_unset!(m, Test(Zero, Zero, TestOp::EitherNonZero));
}

#[test]
fn both_nonzero() {
    let mut m = init_machine(1024 * 4);
    test_unset!(
        m,
        Imm(A, Literal12Bit::new_checked(44).unwrap()),
        Test(A, B, TestOp::BothNonZero)
    );
    m.reset();
    test_unset!(m, Test(Zero, Zero, TestOp::BothNonZero));
    m.reset();
    test_set!(
        m,
        Imm(A, Literal12Bit::new_checked(1).unwrap()),
        Imm(B, Literal12Bit::new_checked(2).unwrap()),
        Test(A, B, TestOp::BothNonZero)
    );
}
