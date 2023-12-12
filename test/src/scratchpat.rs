use crate::registers::Registers;

pub fn init(regs: &Registers) {
    let num_of_buffers = regs
        .capability
        .hcsparams2
        .read_volatile()
        .max_scratchpad_buffers();

    if num_of_buffers > 0 {
        todo!("Implement scratchpad buffer initialization");
    }
}
