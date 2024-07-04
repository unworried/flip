use flipvm::op::Instruction::{self, *};
use flipvm::op::Nibble;
use flipvm::{
    Machine,
    Register::{self, *},
};

const SIGHALT: u8 = 0x01;

fn signal_halt(vm: &mut Machine, _: u16) -> Result<(), String> {
    vm.halt = true;
    Ok(())
}

fn run(vm: &mut Machine, program: &[Instruction]) -> Result<(), String> {
    let program_words: Vec<_> = program.iter().map(|x| x.encode_u16()).collect();
    unsafe {
        let program_bytes = program_words.align_to::<u8>().1;
        vm.memory.load_from_vec(program_bytes, 0);
    }
    vm.set_register(Register::SP, 0x1000);
    vm.define_handler(SIGHALT, signal_halt);
    while !vm.halt {
        vm.step()?;
    }
    Ok(())
}

#[test]
fn test_add() {
    let mut vm = Machine::new(1024 * 4);
    vm.reset();
    let program = vec![
        Imm(A, 11),
        Imm(B, 15),
        Add(A, B, C),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_eq!(vm.get_register(C), 26);
}

#[test]
fn test_sub() {
    let mut vm = Machine::new(1024 * 4);
    vm.reset();
    let program = vec![
        Imm(A, 20),
        Imm(B, 15),
        Sub(A, B, C),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_eq!(vm.get_register(C), 5);
}

#[test]
fn test_sub_overflow() {
    let mut vm = Machine::new(1024 * 4);
    vm.reset();
    let program = vec![
        Imm(A, 1),
        Imm(B, 57),
        Sub(A, B, C),
        System(Zero, Zero, Nibble::new(SIGHALT)),
    ];
    run(&mut vm, &program).unwrap();
    assert_eq!(vm.get_register(C), u16::MAX - 55);
}
