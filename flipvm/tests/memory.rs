use flipvm::op::Instruction::*;
use flipvm::op::{Literal12Bit, Nibble};
use flipvm::Addressable;
use flipvm::Register::*;

use self::common::{init_machine, run, SIGHALT};

mod common;

#[test]
fn load() {
    let mut m = init_machine(1024 * 5);
    m.vm.memory.write2(0x100, 0x77).unwrap();
    m.vm.memory.write2(0x1000, 0x999).unwrap();

    let program = vec![
        Imm(B, Literal12Bit::new_checked(0x100).unwrap()),
        Imm(C, Literal12Bit::new_checked(0x100).unwrap()),
        ShiftLeft(C, C, Nibble::new_checked(4).unwrap()),
        LoadWord(A, B, Zero),
        LoadWord(M, C, Zero),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_reg_eq!(m, A, 0x77);
    assert_reg_eq!(m, M, 0x999);
}

#[test]
fn store() {
    let mut m = init_machine(1024 * 4);

    let program = vec![
        Imm(A, Literal12Bit::new_checked(0x99).unwrap()),
        Imm(B, Literal12Bit::new_checked(0x11).unwrap()),
        StoreWord(B, Zero, A),
        Imm(B, Literal12Bit::new_checked(0x22).unwrap()),
        StoreWord(B, Zero, A),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_mem_eq!(m, 0x11, 0x99);
    assert_mem_eq!(m, 0x22, 0x99);
    assert_mem_eq!(m, 0x30, 0x00);
}
