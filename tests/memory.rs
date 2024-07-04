use flipvm::op::{Instruction::*, Literal12Bit};
use flipvm::op::Nibble;
use flipvm::{Addressable, Machine, Register::*};

use self::common::{run, SIGHALT};

mod common;

#[test]
fn load() {
    let mut vm = Machine::new(1024 * 4);
    vm.memory.write2(0x100, 0x77);
    vm.memory.write2(0x1000, 0x999);

    let program = vec![
        Imm(B, Literal12Bit::new_checked(0x100).unwrap()),
        Imm(C, Literal12Bit::new_checked(0x100).unwrap()),
        ShiftLeft(C, C, Nibble::new(4)),
        Load(A, B, Zero),
        Load(M, C, Zero),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, A, 0x77);
    assert_reg_eq!(vm, M, 0x999);
}

#[test]
fn store() {
    let mut vm = Machine::new(1024 * 4);

    let program = vec![
        Imm(A, Literal12Bit::new_checked(0x99).unwrap()),
        Imm(B, Literal12Bit::new_checked(0x11).unwrap()),
        Store(B, Zero, A),
        Imm(B, Literal12Bit::new_checked(0x22).unwrap()),
        Store(B, Zero, A),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_mem_eq!(vm, 0x11, 0x99);
    assert_mem_eq!(vm, 0x22, 0x99);
    assert_mem_eq!(vm, 0x30, 0x00);
}
