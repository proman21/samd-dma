use core::u16;
use core::ptr;
use crate::{
    StepSize,
    BlockAction,
    EventOutput,
};

bitflags! {
    #[repr(transparent)]
    #[derive(Default)]
    pub(crate) struct RawBlockTransferCtrl: u16 {
        const STEPSIZE_2 = 0x8000;
        const STEPSIZE_1 = 0x4000;
        const STEPSIZE_0 = 0x2000;
        const STEPSEL    = 0x1000;
        const DSTINC     = 0x0800;
        const SRCINC     = 0x0400;
        const BEATSIZE_1 = 0x0200;
        const BEATSIZE_0 = 0x0100;
        const BLOCKACT_1 = 0x0010;
        const BLOCKACT_0 = 0x0008;
        const EVOSEL_1   = 0x0004;
        const EVOSEL_0   = 0x0002;
        const VALID      = 0x0001;
    }
}

/// The raw descriptor memory structure used by the DMA system to configure a block transfer.
#[repr(C)]
#[derive(Debug)]
pub struct TransferDescriptor {
    btctrl: RawBlockTransferCtrl,
    btcnt: u16,
    srcaddr: *const (),
    dstaddr: *const (),
    descaddr: *mut TransferDescriptor,
}

impl Default for TransferDescriptor {
    fn default() -> Self {
        TransferDescriptor {
            srcaddr: ptr::null(),
            dstaddr: ptr::null(),
            descaddr: ptr::null_mut(),
            ..Default::default()
        }
    }
}

impl TransferDescriptor {
    /// Create a new empty descriptor.
    pub fn new() -> TransferDescriptor {
        Default::default()
    }

    /// Get the type-erased source address.
    pub fn get_src_addr(&self) -> *const () {
        self.srcaddr
    }

    /// Get the type-erased destination address.
    pub fn get_dst_addr(&self) -> *const () {
        self.dstaddr
    }

    /// Get address for the next linked descriptor.
    pub fn get_next_desc_addr(&self) -> *mut TransferDescriptor {
        self.descaddr
    }

    /// Set the source address of the descriptor. This is a type erased pointer.
    pub fn set_src_addr(&mut self, addr: *const ()) {
        self.srcaddr = addr;
    }

    /// Set the destination address of the descriptor. This is a type erased pointer.
    pub fn set_dst_addr(&mut self, addr: *const ()) {
        self.dstaddr = addr;
    }

    /// Mark the descriptor as valid.
    pub fn set_valid(&mut self) {
        self.btctrl.insert(RawBlockTransferCtrl::VALID);
    }

    /// Mark the descriptor as invalid.
    pub fn set_invalid(&mut self) {
        self.btctrl.remove(RawBlockTransferCtrl::VALID);
    }

    /// Return the value of the valid bit.
    pub fn is_valid(&self) -> bool {
        self.btctrl.intersects(RawBlockTransferCtrl::VALID)
    }

    /// Set the block count for the descriptor.
    pub fn set_block_count(&mut self, block_count: u16) {
        self.btcnt = block_count;
    }

    /// Get the configured block transfer count.
    pub fn get_block_transfer_count(&self) -> u16 {
        self.btcnt
    }

    /// Get the step size of the descriptor.
    pub fn get_step_size(&self) -> StepSize {
        StepSize::from((self.btctrl.bits() & 0xe000) >> 13)
    }

    /// Set the step size for the descriptor.
    pub fn set_step_size(&mut self, step_size: StepSize) {
        let value = step_size as u8;
        self.btctrl.set(RawBlockTransferCtrl::STEPSIZE_2, value & 0b100 != 0);
        self.btctrl.set(RawBlockTransferCtrl::STEPSIZE_1, value & 0b010 != 0);
        self.btctrl.set(RawBlockTransferCtrl::STEPSIZE_0, value & 0b001 != 0);
    }

    /// Get which address is incremented with the descriptor's step size, if enabled.
    /// 
    /// Returns `true` if the source address is incremented, `false` if the destination address is incremented.
    pub fn get_step_selection(&self) -> bool {
        self.btctrl.intersects(RawBlockTransferCtrl::STEPSEL)
    }

    /// Set which address is incremented with the descriptor's step size, if enabled.
    /// 
    /// `true` will increment the source address by the step size, `false` will increment the destination address by 
    /// the step size.
    pub fn set_step_selection(&mut self, step_sel: bool) {
        self.btctrl.set(RawBlockTransferCtrl::STEPSEL, step_sel);
    }

    /// Get whether the destination address is incremented after each beat transfer.
    /// 
    /// If [`get_step_selection()`] is `true`, the address will be incremented by 1. Otherwise, the address will be 
    /// incremented by the step size.
    /// 
    /// [`get_step_selection()`]: #method.get_step_selection
    pub fn get_dest_addr_increment(&self) -> bool {
        self.btctrl.intersects(RawBlockTransferCtrl::DSTINC)
    }

    /// Set whether the destination address is incremented after each beat transfer.
    pub fn set_dest_addr_increment(&mut self, increment: bool) {
        self.btctrl.set(RawBlockTransferCtrl::DSTINC, increment);
    }

    /// Get whether the source address is incremented after each beat transfer.
    /// 
    /// If [`get_step_selection()`] is `false`, the address will be incremented by 1. Otherwise, the address will be 
    /// incremented by the step size.
    /// 
    /// [`get_step_selection()`]: #method.get_step_selection
    pub fn get_src_addr_increment(&self) -> bool {
        self.btctrl.intersects(RawBlockTransferCtrl::SRCINC)
    }

    /// Set whether the source address is incremented after each beat transfer.
    pub fn set_src_addr_increment(&mut self, increment: bool) {
        self.btctrl.set(RawBlockTransferCtrl::SRCINC, increment);
    }

    /// Get the action taken after this block transfer completes.
    pub fn get_block_action(&self) -> BlockAction {
        BlockAction::from((self.btctrl.bits() & 0x18) >> 3)
    }

    /// Set the action taken after this block transfer completes.
    pub fn set_block_action(&mut self, block_action: BlockAction) {
        let value = block_action as u8;
        self.btctrl.set(RawBlockTransferCtrl::BLOCKACT_1, value & 0b10 != 0);
        self.btctrl.set(RawBlockTransferCtrl::BLOCKACT_0, value & 0b01 != 0);
    }

    /// Get the trigger that causes the transfer to output an event.
    pub fn get_event_output(&self) -> EventOutput {
        EventOutput::from((self.btctrl.bits() & 0x18) >> 3)
    }

    /// Set the trigger that causes the transfer to output an event.
    pub fn set_event_output(&mut self, event_output: EventOutput) {
        let value = event_output as u8;
        self.btctrl.set(RawBlockTransferCtrl::EVOSEL_1, value & 0b10 != 0);
        self.btctrl.set(RawBlockTransferCtrl::EVOSEL_0, value & 0b01 != 0);
    }

    /// Link a transfer descriptor to execute AFTER this descriptor.
    pub fn link_descriptor(&mut self, next: *mut TransferDescriptor) {
        self.descaddr = next;
    }

    /// Unlink the next transfer descriptor, returning its address (which maybe null).
    pub fn unlink_descriptor(&mut self) -> *mut TransferDescriptor {
        let next = self.descaddr;
        self.descaddr = ptr::null_mut();
        next
    }
}