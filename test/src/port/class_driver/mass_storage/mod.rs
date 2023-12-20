mod scsi;

use crate::{
    page_box::PageBox,
    port::init::fully_operational::FullyOperational,
    structures::descriptor::{Configuration, Descriptor},
};
use alloc::vec::Vec;
use log::info;
use scsi::{
    command_data_block,
    response::{Inquiry, Read10, ReadCapacity10},
    CommandBlockWrapper, CommandBlockWrapperHeaderBuilder, CommandStatusWrapper,
};
use xhci::context::EndpointType;

pub(in crate::port) async fn task(eps: FullyOperational) {
    let mut m = MassStorage::new(eps);
    info!("This is the task of USB Mass Storage.");

    m.configure().await;
    info!("USB Mass Storage is configured.");

    let b = m.inquiry().await;
    info!("Inquiry Command: {:?}", b);

    let b = m.read_capacity_10().await;
    info!("Read Capacity: {:?}", b);

    m.read10().await;

    m.write10().await;
}

struct MassStorage {
    ep: FullyOperational,
}
impl MassStorage {
    fn new(ep: FullyOperational) -> Self {
        Self { ep }
    }

    async fn configure(&mut self) {
        let d = self.configuration_descriptor();
        self.ep.set_configure(d.config_val()).await;
    }

    fn configuration_descriptor(&self) -> Configuration {
        *self
            .ep
            .descriptors()
            .iter()
            .filter_map(|x| {
                if let Descriptor::Configuration(c) = x {
                    Some(c)
                } else {
                    None
                }
            })
            .collect::<Vec<&Configuration>>()[0]
    }

    async fn inquiry(&mut self) -> Inquiry {
        const LEN: u16 = 0x24;

        let header = CommandBlockWrapperHeaderBuilder::default()
            .transfer_length(LEN.into())
            .flags(scsi::Flags::In)
            .lun(0)
            .command_len(6)
            .build()
            .expect("Failed to build an inquiry command block wrapper.");
        let data = command_data_block::Inquiry::new(LEN);
        let mut wrapper = PageBox::from(CommandBlockWrapper::new(header, data.into()));

        let (response, status): (PageBox<Inquiry>, _) = self.send_scsi_command(&mut wrapper).await;

        status.check_corruption();
        *response
    }

    async fn read_capacity_10(&mut self) -> ReadCapacity10 {
        let header = CommandBlockWrapperHeaderBuilder::default()
            .transfer_length(8)
            .flags(scsi::Flags::In)
            .lun(0)
            .command_len(10)
            .build()
            .expect("Failed to build a read capacity command block wrapper");
        let data = command_data_block::ReadCapacity::default();
        let mut wrapper = PageBox::from(CommandBlockWrapper::new(header, data.into()));

        let (response, status): (PageBox<ReadCapacity10>, _) =
            self.send_scsi_command(&mut wrapper).await;

        status.check_corruption();
        *response
    }

    async fn read10(&mut self) -> PageBox<Read10> {
        let header = CommandBlockWrapperHeaderBuilder::default()
            .transfer_length(0x8000)
            .flags(scsi::Flags::In)
            .lun(0)
            .command_len(0x0a)
            .build()
            .expect("Failed to build a read 10 command block wrapper.");
        let data = command_data_block::Read10::new(0, 64);
        let mut wrapper = PageBox::from(CommandBlockWrapper::new(header, data.into()));

        let (response, status): (PageBox<Read10>, _) = self.send_scsi_command(&mut wrapper).await;

        status.check_corruption();
        response
    }

    async fn write10(&mut self) {
        let header = CommandBlockWrapperHeaderBuilder::default()
            .transfer_length(0x0008)
            .flags(scsi::Flags::Out)
            .lun(0)
            .command_len(0x0a)
            .build()
            .expect("Failed to build a write 10 command block wrapper.");
        let data = command_data_block::Write10::new(0, 64);
        let mut wrapper = PageBox::from(CommandBlockWrapper::new(header, data.into()));

        let content = PageBox::from(0x334_usize);

        let status = self.send_scsi_command_for_out(&mut wrapper, &content).await;
        status.check_corruption();
    }

    async fn send_scsi_command<T>(
        &mut self,
        c: &mut PageBox<CommandBlockWrapper>,
    ) -> (PageBox<T>, PageBox<CommandStatusWrapper>)
    where
        T: Default,
    {
        self.send_command_block_wrapper(c).await;
        let response = self.receive_command_response().await;
        let status = self.receive_command_status().await;
        (response, status)
    }

    async fn send_scsi_command_for_out(
        &mut self,
        c: &mut PageBox<CommandBlockWrapper>,
        d: &PageBox<impl ?Sized>,
    ) -> PageBox<CommandStatusWrapper> {
        self.send_command_block_wrapper(c).await;
        self.send_additional_data(d).await;
        self.receive_command_status().await
    }

    async fn send_command_block_wrapper(&mut self, c: &mut PageBox<CommandBlockWrapper>) {
        self.ep
            .issue_normal_trb(c, EndpointType::BulkOut)
            .await
            .expect("Failed to send a SCSI command.");
    }

    async fn receive_command_response<T>(&mut self) -> PageBox<T>
    where
        T: Default,
    {
        let c = PageBox::default();
        self.ep
            .issue_normal_trb(&c, EndpointType::BulkIn)
            .await
            .expect("Failed to receive a SCSI command reponse.");
        c
    }

    async fn send_additional_data(&mut self, d: &PageBox<impl ?Sized>) {
        self.ep
            .issue_normal_trb(d, EndpointType::BulkOut)
            .await
            .expect("Failed to send a data.");
    }

    async fn receive_command_status(&mut self) -> PageBox<CommandStatusWrapper> {
        let b = PageBox::default();
        self.ep
            .issue_normal_trb(&b, EndpointType::BulkIn)
            .await
            .expect("Failed to receive a SCSI status.");
        b
    }
}
