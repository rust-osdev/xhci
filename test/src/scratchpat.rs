use crate::registers;

pub fn init() {
    let num_of_buffers = registers::handle(|r| {
        r.capability
            .hcsparams2
            .read_volatile()
            .max_scratchpad_buffers()
    });

    if num_of_buffers > 0 {
        todo!("Implement scratchpad buffer initialization");
    }
}
