use flipvm::op::{Instruction::*, Literal7Bit, Nibble, TestOp};
use flipvm::Machine;
use flipvm::Register::*;

use self::common::{run, SIGHALT};

mod common;

#[test]
fn loop_control() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, 5),
        // Start = 2
        Test(A, Zero, TestOp::Neq),
        AddIf(PC, PC, Nibble::new(2)),
        Imm(PC, 14),
        AddImmSigned(A, Literal7Bit::from_signed(-1)),
        AddImm(B, Literal7Bit::new(1)),
        Imm(PC, 2),
        // End = 14
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, B, 5);
}
