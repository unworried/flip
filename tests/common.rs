use flipvm::op::Instruction;
use flipvm::{Machine, Register};

pub const SIGHALT: u8 = 0x01;

fn signal_halt(vm: &mut Machine, _: u16) -> Result<(), String> {
    vm.halt = true;
    Ok(())
}

pub fn run(vm: &mut Machine, program: &[Instruction]) -> Result<(), String> {
    vm.reset();
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
