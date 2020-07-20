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
//! Because of the design of the DMA system on the SAMD21 family, any channel methods that modify channel registers are
//! NOT interrupt-safe. Beware of accessing or mutating channel register without calling in an interrupt-free section.
#![no_std]
#![deny(missing_docs)]

#[cfg(not(any(feature = "samd5x", feature = "samd21")))]
compile_error!("Please use this crate's feature flags to select a SAM micro-controller to target.");

#[macro_use]
extern crate bitflags;
extern crate smart_default;

#[cfg(feature = "samd51j19a")]
use atsamd51j19a as target_device;

#[cfg(feature = "samd51j20a")]
use atsamd51j20a as target_device;

#[cfg(feature = "samd51g19a")]
use atsamd51g19a as target_device;

#[cfg(feature = "samd21g18a")]
use atsamd21g18a as target_device;

#[cfg(feature = "samd21e18a")]
use atsamd21e18a as target_device;

#[cfg(feature = "samd21j18a")]
use atsamd21j18a as target_device;

mod channel;
mod types;
mod descriptors;
pub mod storage;
pub mod consts {
    //! Contains types used to identify DMA channels.
    #![allow(missing_docs)]
    use typenum::consts::*;

    pub type CH0 = U0;
    pub type CH1 = U1;
    pub type CH2 = U2;
    pub type CH3 = U3;
    pub type CH4 = U4;
    pub type CH5 = U5;
    pub type CH6 = U6;
    pub type CH7 = U7;
    pub type CH8 = U8;
    pub type CH9 = U9;
    pub type CH10 = U10;
    pub type CH11 = U11;

    #[cfg(feature = "samd5x")]
    mod samd5x {
        use typenum::consts::*;

        pub type CH12 = U12;
        pub type CH13 = U13;
        pub type CH14 = U14;
        pub type CH15 = U15;
        pub type CH16 = U16;
        pub type CH17 = U17;
        pub type CH18 = U18;
        pub type CH19 = U19;
        pub type CH20 = U20;
        pub type CH21 = U21;
        pub type CH22 = U22;
        pub type CH23 = U23;
        pub type CH24 = U24;
        pub type CH25 = U25;
        pub type CH26 = U26;
        pub type CH27 = U27;
        pub type CH28 = U28;
        pub type CH29 = U29;
        pub type CH30 = U30;
        pub type CH31 = U31;
    }
    
    #[cfg(feature = "samd5x")]
    pub use self::samd5x::*;
}

#[allow(unused_imports)]
use core::u32;
use core::u16;
use target_device::DMAC;
use typenum::consts::*;
use typenum::{Unsigned, IsLess};
use storage::DmaStorage;

pub use self::channel::*;
pub use self::types::*;
pub use self::descriptors::*;

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
        dmac.baseaddr.write(|w| unsafe { w.bits(storage.baseaddr() as u32) });
        dmac.wrbaddr.write(|w| unsafe { w.bits(storage.wbaddr() as u32) });
        DMAController {
            #[cfg(feature = "samd21")]
            channels: u16::MAX >> 16 - T::Size::U16,
            #[cfg(feature = "samd5x")]
            channels: u32::MAX >> 32 - T::Size::U32,
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

    /// Get the value of the "Run While Debug" setting.
    pub fn get_run_while_debug(&self) -> bool {
        self.dmac.dbgctrl.read().dbgrun().bit()
    }

    /// Set the value of the "Run While Debug" setting.
    pub fn set_run_while_debug(&mut self, val: bool) {
        self.dmac.dbgctrl.write(|w| w.dbgrun().bit(val));
    } 

    /// Take a DMA channel. If the channel has already been taken or if the ID is not available,
    /// `None` is returned. If you don't call `return_channel` on this channel, you can never get it back.
    /// 
    /// This method uses the turbofish syntax to specify the channel ID you wish to take, like so:
    /// `take_channel::<CH1>()`.
    /// 
    /// # Safety
    /// 
    /// Taking the same channel from overlapping interrupt contexts could lead to double channels all the way.
    /// This will alias the exclusive references of the base and write-back descriptors corresponding to this channel.
    /// The same problem applies when a call to `return_channel` overlaps with a call to this function for the same 
    /// channel.
    pub fn take_channel<U: Unsigned>(&mut self) -> Option<Channel> where U: IsLess<T::Size, Output = True>{
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

    /// Get the Quality of Service guarantee for the specified priority level.
    #[cfg(feature = "samd5x")]
    pub fn get_priority_qos(&self, level: Priority) -> QoS {
        match level {
            Priority::Level0 => self.dmac.prictrl0.read().qos0().variant().into(),
            Priority::Level1 => self.dmac.prictrl0.read().qos1().variant().into(),
            Priority::Level2 => self.dmac.prictrl0.read().qos2().variant().into(),
            Priority::Level3 => self.dmac.prictrl0.read().qos3().variant().into(),
        }
    }

    /// Set the Quality of Service guarantee for the specified priority level.
    #[cfg(feature = "samd5x")]
    pub fn set_piority_qos(&mut self, level: Priority, qos: QoS) {
        match level {
            Priority::Level0 => self.dmac.prictrl0.modify(|_, w| w.qos0().bits(qos as u8)),
            Priority::Level1 => self.dmac.prictrl0.modify(|_, w| w.qos1().bits(qos as u8)),
            Priority::Level2 => self.dmac.prictrl0.modify(|_, w| w.qos2().bits(qos as u8)),
            Priority::Level3 => self.dmac.prictrl0.modify(|_, w| w.qos3().bits(qos as u8)),
        }
    }

    /// Get the Quality of Service guarantee for data transfer.
    #[cfg(feature = "samd21")]
    pub fn get_data_transfer_qos(&self) -> QoS {
        self.dmac.qosctrl.read().dqos().variant().into()
    }

    /// Get the Quality of Service guarantee for fetching transfer descriptors.
    #[cfg(feature = "samd21")]
    pub fn get_fetch_qos(&self) -> QoS {
        self.dmac.qosctrl.read().fqos().variant().into()
    }

    /// Get the Quality of Service guarantee for writing transfer descriptors to the write-back section.
    #[cfg(feature = "samd21")]
    pub fn get_write_back_qos(&self) -> QoS {
        self.dmac.qosctrl.read().wrbqos().variant().into()
    }

    /// Set the Quality of Service guarantee for data transfer.
    #[cfg(feature = "samd21")]
    pub fn set_data_transfer_qos(&mut self, qos: QoS) {
        self.dmac.qosctrl.modify(|_, w| w.dqos().bits(qos as u8))
    }

    /// Set the Quality of Service guarantee for fetching transfer descriptors.
    #[cfg(feature = "samd21")]
    pub fn set_fetch_qos(&mut self, qos: QoS) {
        self.dmac.qosctrl.modify(|_, w| w.fqos().bits(qos as u8))
    }

    /// Set the Quality of Service guarantee for writing transfer descriptors to the write-back section.
    #[cfg(feature = "samd21")]
    pub fn set_write_back_qos(&mut self, qos: QoS) {
        self.dmac.qosctrl.modify(|_, w| w.wrbqos().bits(qos as u8))
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

    /// Get the interrupt status of all channels.
    pub fn get_channel_interrupt_status(&self) -> Channels {
        Channels::from_bits_truncate(self.dmac.intstatus.read().bits())
    }

    /// Get a bitfield of all pending channels.
    pub fn get_pending_channels(&self) -> Channels {
        Channels::from_bits_truncate(self.dmac.pendch.read().bits().into())
    }

    /// Get a bitfield of all busy channels.
    pub fn get_busy_channels(&self) -> Channels {
        Channels::from_bits_truncate(self.dmac.busych.read().bits().into())
    }

    /// Get ID of the last channel to be granted access to the DMA system.
    pub fn get_active_channel(&self) -> u8 {
        self.dmac.active.read().id().bits()
    }

    /// Send a trigger request to a channel.
    /// 
    /// If the channel is not in a pending state, the request is ignored.
    pub fn trigger_channel(&mut self, id: u8) {
        self.dmac.swtrigctrl.modify(|r, w| unsafe { w.bits(r.bits() | 1 << id) })
    }

    /// Get the block transfer count of the currently active channel, if there is one.
    pub fn get_active_block_transfer_count(&self) -> Option<u16> {
        let reg = self.dmac.active.read();
        if reg.abusy().bit_is_set() {
            Some(reg.btcnt().bits())
        } else {
            None
        }
    }

    /// Return whether the selected priority level has an active request.
    pub fn priority_level_has_request(&self, pri: Priority) -> bool {
        match pri {
            Priority::Level0 => self.dmac.active.read().lvlex0().bit(),
            Priority::Level1 => self.dmac.active.read().lvlex1().bit(),
            Priority::Level2 => self.dmac.active.read().lvlex2().bit(),
            Priority::Level3 => self.dmac.active.read().lvlex3().bit(),
        }
    }

    /// Get the lowest pending interrupt channel's interrupt flags, if present.
    /// 
    /// The ID and interrupt flags are returned.
    pub fn get_lowest_pending_channel_interrupts(&self) -> Option<(u8, Interrupts)> {
        let reg = self.dmac.intpend.read();
        let int = Interrupts::from_bits_truncate((reg.bits() & 0x700).to_be_bytes()[0]);
        if !int.is_empty() {
            Some((reg.id().bits(), int))
        } else {
            None
        }
    }

    /// Get the lowest pending interrupt channel's status, if present.
    /// 
    /// The ID and status are returned.
    pub fn get_lowest_pending_channel_status(&self) -> Option<(u8, Option<Status>)> {
        let reg = self.dmac.intpend.read();
        let int = Interrupts::from_bits_truncate((reg.bits() & 0x700).to_be_bytes()[0]);
        if !int.is_empty() {
            if reg.pend().bit_is_set() {
                return Some((reg.id().bits(), Some(Status::Pending)));
            }
    
            if reg.busy().bit_is_set() {
                return Some((reg.id().bits(), Some(Status::Busy)));
            }
    
            if reg.ferr().bit_is_set() {
                return Some((reg.id().bits(), Some(Status::FetchError)));
            }
    
            #[cfg(feature = "samd5x")]
            if reg.crcerr().bit_is_set() {
                return Some((reg.id().bits(), Some(Status::CRCError)));
            }

            return Some((reg.id().bits(), None));
        } else {
            None
        }
    }

    /// Get the interrupt flags of a particular channel.
    pub fn get_channel_pending_interrupts(&mut self, id: u8) -> Interrupts {
        self.dmac.intpend.modify(|_, w| unsafe { w.id().bits(id) });
        Interrupts::from_bits_truncate((self.dmac.intpend.read().bits() & 0x700).to_be_bytes()[0])
    }

    /// Set the interrupt flags of a particular channel.
    pub fn set_channel_pending_interrupts(&mut self, id: u8, int: Interrupts) {
        self.dmac.intpend.write(|w| unsafe { w.bits(u16::from_be_bytes([int.bits(), id])) })
    }

    /// Get the status of a particular channel.
    pub fn get_channel_status(&mut self, id: u8) -> Option<Status> {
        self.dmac.intpend.modify(|_, w| unsafe { w.id().bits(id) });
        let reg = self.dmac.intpend.read();

        if reg.pend().bit_is_set() {
            return Some(Status::Pending);
        }

        if reg.busy().bit_is_set() {
            return Some(Status::Busy);
        }

        if reg.ferr().bit_is_set() {
            return Some(Status::FetchError);
        }

        #[cfg(feature = "samd5x")]
        if reg.crcerr().bit_is_set() {
            return Some(Status::CRCError);
        }

        None
    }
}