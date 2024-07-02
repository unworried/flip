use rust_vm::Machine;

pub fn main() -> Result<(), String> {
    let mut vm = Machine::new();
    vm.memory.write(0, 0xf);
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()
}
