use core::fmt;
use crate::target_device::DMAC;
use crate::{TriggerSource, TriggerAction, Priority, Interrupts};
#[cfg(feature = "samd5x")]
use crate::{BurstLength, FifoThreshold};
use crate::descriptors::{TransferDescriptor};

#[cfg(feature = "samd5x")]
macro_rules! channel_reg {
    (@arm $reg:ident, $n:literal) => {
        paste::expr! { unsafe { &(*DMAC::ptr()).[<$reg $n>] } }
    };
    ($reg:ident, $n:expr) => {
        match $n {
            0 => channel_reg!(@arm $reg, 0),
            1 => channel_reg!(@arm $reg, 1),
            2 => channel_reg!(@arm $reg, 2),
            3 => channel_reg!(@arm $reg, 3),
            4 => channel_reg!(@arm $reg, 4),
            5 => channel_reg!(@arm $reg, 5),
            6 => channel_reg!(@arm $reg, 6),
            7 => channel_reg!(@arm $reg, 7),
            8 => channel_reg!(@arm $reg, 8),
            9 => channel_reg!(@arm $reg, 9),
            10 => channel_reg!(@arm $reg, 10),
            11 => channel_reg!(@arm $reg, 11),
            12 => channel_reg!(@arm $reg, 12),
            13 => channel_reg!(@arm $reg, 13),
            14 => channel_reg!(@arm $reg, 14),
            15 => channel_reg!(@arm $reg, 15),
            16 => channel_reg!(@arm $reg, 16),
            17 => channel_reg!(@arm $reg, 17),
            18 => channel_reg!(@arm $reg, 18),
            19 => channel_reg!(@arm $reg, 19),
            20 => channel_reg!(@arm $reg, 20),
            21 => channel_reg!(@arm $reg, 21),
            22 => channel_reg!(@arm $reg, 22),
            23 => channel_reg!(@arm $reg, 23),
            24 => channel_reg!(@arm $reg, 24),
            25 => channel_reg!(@arm $reg, 25),
            26 => channel_reg!(@arm $reg, 26),
            27 => channel_reg!(@arm $reg, 27),
            28 => channel_reg!(@arm $reg, 28),
            29 => channel_reg!(@arm $reg, 29),
            30 => channel_reg!(@arm $reg, 30),
            31 => channel_reg!(@arm $reg, 31),
            _ => unreachable!()
        }
    };
}

#[cfg(feature = "samd21")]
macro_rules! channel_reg {
    ($reg:ident, $n:expr) => {
        unsafe {
            (&*DMAC::ptr()).chid.write(|w| w.id().bits($n));
            &(&*DMAC::ptr()).$reg
        }
    };
}

/// Error type for the kinds of errors that can occur during a transaction.
#[derive(Debug)]
pub enum TransactionError {
    /// An invalid descriptor was fetched from memory.
    InvalidDescriptor,
    /// A bus error was detected during a beat transfer.
    TransferError,
    /// The CRC module detected data corruption.
    CRCError,
}

impl fmt::Display for TransactionError {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        use self::TransactionError::*;
        match self {
            InvalidDescriptor => write!(w, "An invalid descriptor was fetched from memory."),
            TransferError => write!(w, "A bus error was detected during a beat transfer."),
            CRCError => write!(w, "The CRC module detected data corruption.")
        }
    }
}

/// The return value of `Transaction::try_wait()`.
pub enum WaitResult {
    /// The transaction has ended or been aborted.
    Done,
    /// The transaction is suspended.
    Suspended,
    /// The transaction is still ongoing.
    Ongoing,
}

/// DMA channel.
/// 
/// This structure represents a DMA channel. Using the [`start_transfer`] method, you can
/// setup a DMA transfer transaction. You can also configure the DMA channel properties.
/// 
/// [`start_transfer`]: #method.start_transfer
pub struct Channel {
    id: u8,
    first_desc: *mut TransferDescriptor,
    write_back: *mut TransferDescriptor,
}

impl Channel {
    pub(crate) fn new(id: u8, first_desc: *mut TransferDescriptor,
        write_back: *mut TransferDescriptor) -> Channel
    {
        Channel {
            id,
            first_desc,
            write_back,
        }
    }

    /// Return the channel ID.
    pub fn id(&self) -> u8 {
        self.id
    }

    ///  Configure whether the channel continues to run in standby.
    #[cfg(feature = "samd5x")]
    pub fn set_run_standby(&mut self, run_standby: bool) {
        channel_reg!(chctrla, self.id).modify(|_, w| w.runstdby().bit(run_standby));
    }

    /// Return true if the channel continues to run in standby.
    #[cfg(feature = "samd5x")]
    pub fn get_run_standby(&self) -> bool {
        channel_reg!(chctrla, self.id).read().runstdby().bit()
    }

    /// Configure how many beats are in a burst.
    #[cfg(feature = "samd5x")]
    pub fn set_burst_length(&mut self, burst_len: BurstLength) {
        channel_reg!(chctrla, self.id).modify(|_, w| w.burstlen().bits(burst_len as u8));
    }

    /// Get the length of a burst in beats.
    #[cfg(feature = "samd5x")]
    pub fn get_burst_length(&self) -> BurstLength {
        channel_reg!(chctrla, self.id).read().burstlen().variant().into()
    }

    /// Set the trigger action for the channel.
    pub fn set_trigger_action(&mut self, trig_act: TriggerAction) {
        #[cfg(feature = "samd5x")]
        let reg = channel_reg!(chctrla, self.id);
        #[cfg(feature = "samd21")]
        let reg = channel_reg!(chctrlb, self.id);
        reg.modify(|_, w| unsafe { w.trigact().bits(trig_act as u8) });
    }

    /// Get the trigger action for the channel.
    pub fn get_trigger_action(&self) -> TriggerAction {
        #[cfg(feature = "samd5x")]
        let reg = channel_reg!(chctrla, self.id);
        #[cfg(feature = "samd21")]
        let reg = channel_reg!(chctrlb, self.id);
        reg.read().trigact().variant().into()
    }

    /// Set threshold for when destination writes occur.
    #[cfg(feature = "samd5x")]
    pub fn set_fifo_threshold(&mut self, fifo_threshold: FifoThreshold) {
        channel_reg!(chctrla, self.id).modify(|_, w| w.threshold().bits(fifo_threshold as u8));
    }

    /// Get the threshold for when destination writes will occur.
    #[cfg(feature = "samd5x")]
    pub fn get_fifi_threshold(&self) -> FifoThreshold {
        channel_reg!(chctrla, self.id).read().threshold().variant().into()
    }

    /// Set the source trigger for the DMA Channel.
    pub fn set_source(&mut self, source: TriggerSource) {
        #[cfg(feature = "samd5x")]
        let reg = channel_reg!(chctrla, self.id);

        #[cfg(feature = "samd21")]
        let reg = channel_reg!(chctrlb, self.id);
        reg.modify(|_, w| unsafe { w.trigsrc().bits(source as u8)});
    }

    /// Get the trigger source for the channel.
    pub fn get_source(&self) -> TriggerSource {
        #[cfg(feature = "samd5x")]
        let reg = channel_reg!(chctrla, self.id);

        #[cfg(feature = "samd21")]
        let reg = channel_reg!(chctrlb, self.id);
        reg.read().trigsrc().variant().into()
    }

    /// Set the priority level of the channel.
    pub fn set_priority(&mut self, priority: Priority) {
        #[cfg(feature = "samd5x")]
        channel_reg!(chprilvl, self.id).write(|w| unsafe { w.prilvl().bits(priority as u8) });
        #[cfg(feature = "samd21")]
        channel_reg!(chctrlb, self.id).modify(|_, w| w.lvl().bits(priority as u8))
    }

    /// Get channel priority level.
    pub fn get_priority(&self) -> Priority {
        #[cfg(feature = "samd5x")]
        return channel_reg!(chprilvl, self.id).read().prilvl().variant().into();
        #[cfg(feature = "samd21")]
        return channel_reg!(chctrlb, self.id).read().lvl().variant().into();
    }

    /// Get a mutable reference to the first descriptor for the channel.
    pub fn get_first_descriptor(&self) -> &mut TransferDescriptor {
        unsafe { &mut *self.first_desc }
    }

    /// Get the channel's interrupt flags.
    pub fn get_interrupt_flags(&self) -> Interrupts {
        Interrupts::from_bits_truncate(channel_reg!(chintflag, self.id).read().bits())
    }

    /// Reset the channel's interrupt flags.
    pub fn clear_interrupt_flags(&mut self) {
        channel_reg!(chintflag, self.id).reset();
    }

    /// Enable interrupts for the channel. Any interrupts that are not set will be disabled.
    pub fn enable_interrupts(&mut self, interrupts: Interrupts) {
        channel_reg!(chintenset, self.id).write(|w| unsafe { w.bits(interrupts.bits()) });
        channel_reg!(chintenclr, self.id).write(|w| unsafe { w.bits(!interrupts.bits()) });
    }

    /// Get the set of enabled channel interrupts.
    pub fn get_enabled_interrupts(&self) -> Interrupts {
        Interrupts::from_bits_truncate(channel_reg!(chintenset, self.id).read().bits())
    }

    /// Read descriptor from the Write-back Address of this channel.
    /// 
    /// # Safety
    /// 
    /// The write-back address of a DMA channel is volatile, as the DMA engine can change it at any time.
    /// As such, reading and writing to this descriptor is unsafe. Act cautiously.
    pub fn get_writeback_descriptor(&mut self) -> *mut TransferDescriptor {
        self.write_back 
    }

    /// Enable the DMA channel.
    /// 
    /// After this call, this channel will be a part of the DMA arbitration scheme (if its corresponding priority level 
    /// is active), and trigger events will cause the transaction to start from the first descriptor.
    pub fn enable(&mut self) {
        channel_reg!(chctrla, self.id).modify(|_, w| w.enable().set_bit());
    }

    /// Return whether the channel is enabled or not.
    pub fn is_enabled(&self) -> bool {
        channel_reg!(chctrla, self.id).read().enable().bit()
    }

    /// Reset the DMA channel. This will set all channel registers to their reset values.
    /// 
    /// If the channel is still enabled (or is in the process of being disabled), this command will be ignored.
    pub fn reset(&mut self) {
        channel_reg!(chctrla, self.id).modify(|_, w| w.swrst().set_bit());
    }

    /// Manually trigger the channel.
    /// 
    /// # Interrupt Safety
    /// This method is not interrupt-safe, and could lead to lost updates.
    /// 
    /// It is the responsibility of the caller to ensure that the call-site is in an interrupt-free section.
    pub fn trigger(&mut self) {
        unsafe {&*DMAC::ptr()}.swtrigctrl.modify(|r, w| unsafe {
            w.bits(r.bits() | (1 << self.id))
        })
    }

    /// Suspend the ongoing transaction. Returns `true` if the command was successfully written, `false` if another 
    /// command is ongoing.
    /// 
    /// This call returns immediately, but the suspend operation won't complete until the ongoing burst transfer 
    /// completes.
    pub fn suspend(&mut self) -> bool {
        let mut ongoing = true;
        channel_reg!(chctrlb, self.id).modify(|r, w| {
            if r.cmd().is_noact() {
                w.cmd().suspend();
                ongoing = false;
            }
            w
        });
        ongoing
    }

    /// Resume the ongoing transaction. Returns `true` if command was successfully written, `false` if another command 
    /// is ongoing.
    pub fn resume(&mut self) -> bool {
        let mut ongoing = true;
        channel_reg!(chctrlb, self.id).modify(|r, w| {
            if r.cmd().is_noact() {
                w.cmd().resume();
                ongoing = false;
            }
            w
        });
        ongoing
    }

    /// Disable the channel. This aborts any ongoing transaction.
    /// 
    /// This call returns immediately, but the transaction will not be aborted until
    /// the ongoing burst transfer completes.
    pub fn disable(&mut self) {
        channel_reg!(chctrla, self.id).modify(|_, w| w.enable().clear_bit());
    }

    /// Returns `true` if a transfer is pending on the channel.
    /// Returns `false` if a channel trigger action is completed, a bus error is detected, or the channel is disabled.
    pub fn is_pending(&self) -> bool {
        channel_reg!(chstatus, self.id).read().pend().bit()
    }

    /// Returns `true` if the channel has started a transfer.
    /// Returns `false` if a channel trigger action is started, a bus error is detected, or the channel is disabled.
    pub fn is_busy(&self) -> bool {
        channel_reg!(chstatus, self.id).read().busy().bit()
    }

    /// Poll the channel to determine the status of the transaction.
    /// 
    /// This will read and reset the interrupt flag registers of the channel to determine the status of the channel.
    /// Therefore it is expected to be called in the appropriate interrupt vector.
    /// 
    /// Any non-error state will return `Ok(WaitResult)`.
    /// Any errors will be returned as `Err(TransactionError)`.
    pub fn poll_status(&mut self) -> Result<WaitResult, TransactionError> {
        let intflag = self.get_interrupt_flags();
        let status = channel_reg!(chstatus, self.id).read();
        self.clear_interrupt_flags();

        if channel_reg!(chctrla, self.id).read().enable().bit_is_set() {
            if intflag.intersects(Interrupts::TERR) {
                #[cfg(feature = "samd5x")]
                if status.crcerr().bit_is_set() {
                    return Err(TransactionError::CRCError);
                }
                return Err(TransactionError::TransferError);
            } else {
                return Ok(WaitResult::Done);
            }
        }

        if intflag.intersects(Interrupts::SUSP) {
            if status.ferr().bit_is_set() {
                return Err(TransactionError::InvalidDescriptor);
            } else {
                return Ok(WaitResult::Suspended)
            }
        }

        Ok(WaitResult::Ongoing)
    } 
}