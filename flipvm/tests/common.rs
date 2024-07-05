use flipvm::op::Instruction;
use flipvm::{Addressable, LinearMemory};
use flipvm::{Machine, Register};

pub const SIGHALT: u8 = 0x01;

fn signal_halt(vm: &mut Machine, _: u16) -> Result<(), String> {
    vm.halt = true;
    Ok(())
}

pub fn run(vm: &mut Machine, program: &[Instruction]) -> Result<(), String> {
    let program_words: Vec<_> = program.iter().map(|x| x.encode_u16()).collect();
    unsafe {
        let program_bytes = program_words.align_to::<u8>().1;
        vm.memory.load_from_vec(program_bytes, 0).unwrap();
    }
    vm.set_register(Register::SP, 1024 * 3);
    vm.define_handler(SIGHALT, signal_halt);
    while !vm.halt {
        vm.step()?;
    }
    Ok(())
}

pub fn init_vm(mem_size: usize) -> Machine {
    let mut vm = Machine::new();
    vm.map(0x0, mem_size, Box::new(LinearMemory::new(mem_size)))
        .unwrap();
    vm
}

#[macro_export]
macro_rules! assert_reg_eq {
    ($vm:expr, $reg:expr, $val:expr) => {
        assert_eq!(
            $vm.get_register($reg),
            $val,
            "expected {} = 0x{:X}, got 0x{:X}",
            stringify!($reg),
            $val,
            $vm.get_register($reg)
        );
    };
}

#[macro_export]
macro_rules! assert_mem_eq {
    ($vm:expr, $reg:ident - $ptr:literal, $val:expr) => {
        let addr = ($vm.get_register($reg) - $ptr) as u32;
        let result = $vm.memory.read2(addr).unwrap();
        assert_eq!(
            result, $val,
            "expected 0x{:X} @ {:X}, got 0x{:X}",
            $val, addr, result
        );
    };

    ($vm:expr, $addr:expr, $val:expr) => {
        let result = $vm.memory.read2(($addr) as u32).unwrap();
        assert_eq!(
            result, $val,
            "expected 0x{:X} @ {:X}, got 0x{:X}",
            $val, $addr, result
        );
    };
}

#[macro_export]
macro_rules! assert_flag_set {
    ($vm:expr, $flag:expr) => {
        assert!(
            $vm.test_flag($flag),
            "expected flag {} to be set",
            stringify!($flag)
        );
    };
}

#[macro_export]
macro_rules! assert_flag_unset {
    ($vm:expr, $flag:expr) => {
        assert!(
            !$vm.test_flag($flag),
            "expected flag {} to be unset",
            stringify!($flag)
        );
    };
}
