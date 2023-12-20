use super::slot_structures_initializer::SlotStructuresInitializer;
use crate::structures::registers;
use xhci::registers::PortRegisterSet;

pub(super) struct Resetter {
    port_number: u8,
}
impl Resetter {
    pub(super) fn new(port_number: u8) -> Self {
        Self { port_number }
    }

    pub(super) fn port_number(&self) -> u8 {
        self.port_number
    }

    pub(super) async fn reset(self) -> SlotStructuresInitializer {
        self.start_resetting();
        self.wait_until_reset_is_completed();
        SlotStructuresInitializer::new(self).await
    }

    fn start_resetting(&self) {
        self.update_port_register(|r| {
            r.portsc.set_port_reset();
        });
    }

    fn wait_until_reset_is_completed(&self) {
        while !self.reset_completed() {}
    }

    fn reset_completed(&self) -> bool {
        self.read_port_register(|r| r.portsc.port_reset_change())
    }

    fn read_port_register<T, U>(&self, f: T) -> U
    where
        T: FnOnce(&PortRegisterSet) -> U,
    {
        registers::handle(|r| {
            f(&r.port_register_set
                .read_volatile_at((self.port_number - 1).into()))
        })
    }

    fn update_port_register<T>(&self, f: T)
    where
        T: FnOnce(&mut PortRegisterSet),
    {
        registers::handle(|r| {
            r.port_register_set
                .update_volatile_at((self.port_number - 1).into(), f)
        })
    }
}
