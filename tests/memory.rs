use flipvm::op::Instruction::*;
use flipvm::op::Nibble;
use flipvm::{Machine, Register::*};

use self::common::{run, SIGHALT};

mod common;

#[test]
fn load() {
    let mut vm = Machine::new(1024 * 4);
    vm.memory.write2(0x100, 0x77);
    vm.memory.write2(0x1000, 0x999);

    let program = vec![
        Imm(B, 0x100),
        Imm(C, 0x100),
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
        Imm(A, 0x99),
        Imm(B, 0x11),
        Store(B, Zero, A),
        Imm(B, 0x22),
        Store(B, Zero, A),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_mem_eq!(vm, 0x11, 0x99);
    assert_mem_eq!(vm, 0x22, 0x99);
    assert_mem_eq!(vm, 0x30, 0x00);
}
