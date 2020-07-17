//! DMA library for Microchip SAM micro-controllers.
//! 
//! This library provides a convenience wrapper around the DMA and CRC subsystem of the SAM family of micro-controllers.
//! It is designed to be maximally expressive while providing a convenient interface for simple tasks.
//! 
//! # Safety
//! 
//! The primary goal of this library is to abstract away directly writing to registers. It is not concerned with larger 
//! goals like totally memory safe DMA, which is left to the discretion of the library user and the goals you are 
//! trying to accomplish. If you only need one descriptor per channel and aren't reading from the write-back address, 
//! you can use this library without any unsafe sections. More advanced features of the DMA system are not protected by 
//! this library, and it is possible to shot yourself in the foot if not careful. I highly recommend reading the 
//! relevant sections of the manual for your family of micro-controller to understand how the DMA system works and
//! operates on memory outside of the compilers knowledge.
//! 
//! # SAMD21
//! 
//! 
#![no_std]

#[cfg(not(any(feature = "samd5x", feature = "samd21")))]
compile_error!("Please use this crate's feature flags to select a SAM micro-controller to target.");

#[macro_use]
extern crate bitflags;
extern crate smart_default;

#[cfg(feature = "samd51j19a")]
use atsamd51j19a as target_device;

#[cfg(feature = "samd21j18a")]
use atsamd21j18a as target_device;

mod channel;
mod types;
mod descriptors;
pub mod storage;

#[allow(unused_imports)]
use core::u32;
#[allow(unused_imports)]
use core::u16;
use target_device::DMAC;
use typenum::consts::*;
use typenum::{Unsigned, IsLess};
use storage::DmaStorage;

pub use self::channel::*;
pub use self::types::*;
pub use self::descriptors::*;
pub use typenum::consts;

/// DMA system controller.
/// 
/// Used to distribute channels, as well as control higher level operations of the DMA system.
pub struct DMAController<T: 'static + DmaStorage> {
    #[cfg(feature = "samd5x")]
    channels: u32,
    #[cfg(feature = "samd21")]
    channels: u16,
    storage: &'static mut T,
    dmac: DMAC,
}

impl<T: 'static + DmaStorage> DMAController<T> {
    /// Initialise the DMA Controller with the specified storage.
    pub fn init(dmac: DMAC, storage: &'static mut T) -> DMAController<T> {
        DMAController {
            #[cfg(feature = "samd21")]
            channels: u16::MAX >> 16 - T::Index::U16,
            #[cfg(feature = "samd5x")]
            channels: u32::MAX >> 32 - T::Index::U32,
            storage,
            dmac
        }
    }

    /// Disable all channels and the CRC module. This will abort any ongoing DMA transactions.
    /// The DMA system will not be fully disabled until any ongoing burst transfer is completed.
    pub fn disable(&mut self) {
        self.dmac.ctrl.modify(|_, w| w.dmaenable().clear_bit());
    }

    /// Enable the DMA system.
    pub fn enable(&mut self) {
        self.dmac.ctrl.modify(|r, w| if r.dmaenable().bit_is_clear() {
            w.dmaenable().set_bit()
        } else {
            w
        });
    }

    /// Returns true if the DMA system is enabled.
    pub fn is_enabled(&self) -> bool {
        self.dmac.ctrl.read().dmaenable().bit_is_set()
    }

    /// Take a DMA channel by its number. If the channel has already been taken or if the ID is not available,
    /// `None` is returned. If you don't call `return_channel` on this channel, you can never get it back.
    /// 
    /// # Safety
    /// 
    /// Taking the same channel from overlapping interrupt contexts could lead to double channels all the way.
    /// This will alias the exclusive references of the base and write-back descriptors corresponding to this channel.
    /// The same problem applies when a call to `return_channel` overlaps with a call to this function for the same 
    /// channel.
    pub fn take_channel<U: Unsigned>(&mut self) -> Option<Channel> where U: IsLess<T::Index, Output = True>{
        if self.channels & (1 << U::USIZE) == 0 {
            None
        } else {
            self.channels |= 1 << U::USIZE;
            unsafe {
                Some(Channel::new(U::U8,
                    self.storage.baseaddr().offset(U::to_isize()) as *mut TransferDescriptor,
                    self.storage.wbaddr().offset(U::to_isize()) as *mut TransferDescriptor))
            }
        }
    }

    /// Return a channel to the controller. This will disable the channel and reset the channel and first transfer 
    /// descriptor.
    /// 
    /// # Safety
    /// 
    /// The same interrupt safety issues with `take_channel` apply here as well.
    pub fn return_channel(&mut self, mut channel: Channel) {
        channel.disable();
        channel.reset();
        self.channels &= !(1 << channel.id());
    }

    /// Allow channels with the corresponding priority level to be part of arbitration.
    pub fn enable_priority_level(&mut self, level: Priority) {
        self.set_priority_level(level, true);
    }

    /// Deny channels with the corresponding priority level to be a part of arbitration.
    pub fn diable_priority_level(&mut self, level: Priority) {
        self.set_priority_level(level, false);
    }

    fn set_priority_level(&mut self, level: Priority, value: bool) {
        self.dmac.ctrl.modify(|_, w| match level {
            Priority::Level0 => w.lvlen0().bit(value),
            Priority::Level1 => w.lvlen1().bit(value),
            Priority::Level2 => w.lvlen2().bit(value),
            Priority::Level3 => w.lvlen3().bit(value),
        })
    }

    /// Return true if the priority level is enabled.
    pub fn priority_level_enabled(&self, level: Priority) -> bool {
        match level {
            Priority::Level0 => self.dmac.ctrl.read().lvlen0().bit(),
            Priority::Level1 => self.dmac.ctrl.read().lvlen1().bit(),
            Priority::Level2 => self.dmac.ctrl.read().lvlen2().bit(),
            Priority::Level3 => self.dmac.ctrl.read().lvlen3().bit(),
        }
    }

    /// Enable or disable round-robin scheduling method for channels of the given priority level.
    /// Disabling round-robin scheduling will enable static scheduling.
    pub fn set_priority_level_scheduling(&mut self, level: Priority, enable: bool) {
        self.dmac.prictrl0.modify(|_, w| match level {
            Priority::Level0 => w.rrlvlen0().bit(enable),
            Priority::Level1 => w.rrlvlen1().bit(enable),
            Priority::Level2 => w.rrlvlen2().bit(enable),
            Priority::Level3 => w.rrlvlen3().bit(enable),
        })
    }
}