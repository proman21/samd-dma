use smart_default::SmartDefault;

use crate::target_device::generic::Variant;
#[cfg(feature = "samd5x")]
use crate::target_device::dmac::chctrla::{TRIGACT_A, BURSTLEN_A, THRESHOLD_A, TRIGSRC_A};
#[cfg(feature = "samd21")]
use crate::target_device::dmac::chctrlb::{TRIGACT_A, TRIGSRC_A, LVL_A as PRILVL_A};
#[cfg(feature = "samd5x")]
use crate::target_device::dmac::chprilvl::PRILVL_A;

bitflags! {
    pub struct Interrupts: u8 {
        const TransferError = 0x1;
        const TransferComplete = 0x2;
        const Suspend = 0x4;
    }
}

/// Priority level of a channel.
pub enum Priority {
    Level0 = 0,
    Level1,
    Level2,
    Level3,
}

#[cfg(feature = "samd5x")]
impl From<Variant<u8, PRILVL_A>> for Priority {
    fn from(value: Variant<u8, PRILVL_A>) -> Priority {
        use self::PRILVL_A::*;
        use self::Priority::*;
        match value {
            Variant::Val(LVL0) => Level0,
            Variant::Val(LVL1) => Level1,
            Variant::Val(LVL2) => Level2,
            Variant::Val(LVL3) => Level3,
            _ => Level0,
        }
    }
}

#[cfg(feature = "samd21")]
impl From<PRILVL_A> for Priority {
    fn from(value: PRILVL_A) -> Priority {
        use self::PRILVL_A::*;
        use self::Priority::*;
        match value {
            LVL0 => Level0,
            LVL1 => Level1,
            LVL2 => Level2,
            LVL3 => Level3
        }
    }
}

pub enum QoS {
    Disable = 0,
    Low,
    Medium,
    Critical,
}

/// Length of a burst in beats.
#[cfg(feature = "samd5x")]
pub enum BurstLength {
    Single = 0,
    TwoBeats,
    ThreeBeats,
    FourBeats,
    FiveBeats,
    SixBeats,
    SevenBeats,
    EightBeats,
    NineBeats,
    TenBeats,
    ElevenBeats,
    TwelveBeats,
    ThirteenBeats,
    FourteenBeats,
    FifteenBeats,
    SixteenBeats,
}

#[cfg(feature = "samd5x")]
impl From<BURSTLEN_A> for BurstLength {
    fn from(value: BURSTLEN_A) -> BurstLength {
        use self::BurstLength::*;
        match value {
            BURSTLEN_A::SINGLE => Single,
            BURSTLEN_A::_2BEAT => TwoBeats,
            BURSTLEN_A::_3BEAT => ThreeBeats,
            BURSTLEN_A::_4BEAT => FourBeats,
            BURSTLEN_A::_5BEAT => FiveBeats,
            BURSTLEN_A::_6BEAT => SixBeats,
            BURSTLEN_A::_7BEAT => SevenBeats,
            BURSTLEN_A::_8BEAT => EightBeats,
            BURSTLEN_A::_9BEAT => NineBeats,
            BURSTLEN_A::_10BEAT => TenBeats,
            BURSTLEN_A::_11BEAT => ElevenBeats,
            BURSTLEN_A::_12BEAT => TwelveBeats,
            BURSTLEN_A::_13BEAT => ThirteenBeats,
            BURSTLEN_A::_14BEAT => FourteenBeats,
            BURSTLEN_A::_15BEAT => FifteenBeats,
            BURSTLEN_A::_16BEAT => SixteenBeats,
        }
    }
}

/// What action occurs when a trigger is received.
pub enum TriggerAction {
    /// Trigger starts a block transfer.
    Block = 0,
    /// Trigger starts a burst transfer.
    #[cfg(feature = "samd5x")]
    Burst = 2,
    /// Trigger starts a beat transfer.
    #[cfg(feature = "samd21")]
    Beat = 2,
    /// Trigger starts a transaction transfer.
    Transaction = 3,
}

impl From<Variant<u8, TRIGACT_A>> for TriggerAction {
    fn from(value: Variant<u8, TRIGACT_A>) -> TriggerAction {
        use self::TRIGACT_A::*;
        match value {
            Variant::Val(v) => match v {
                BLOCK => TriggerAction::Block,
                #[cfg(feature = "samd5x")]
                BURST => TriggerAction::Burst,
                #[cfg(feature = "samd21")]
                BEAT => TriggerAction::Beat,
                TRANSACTION => TriggerAction::Transaction,
            },
            Variant::Res(_) => TriggerAction::Block
        }
    }
}

/// Trigger source for a channel.
pub enum TriggerSource {
    Disable = 0,
    RtcTimestamp,
    DsuDcc0,
    DsuDcc1,
    Sercom0Rx,
    Sercom0Tx,
    Sercom1Rx,
    Sercom1Tx,
    Sercom2Rx,
    Sercom3Tx,
    Sercom4Rx,
    Sercom4Tx,
    Sercom5Rx,
    Sercom5Tx,
    Sercom6Rx,
    Sercom6Tx,
    Sercom7Rx,
    Sercom7Tx,
    Can0Debug,
    Can1Debug,
    Tcc0Ovf,
    Tcc0Mc0,
    Tcc0Mc1,
    Tcc0Mc2,
    Tcc0Mc3,
    Tcc0Mc4,
    Tcc0Mc5,
    Tcc1Ovf,
    Tcc1Mc0,
    Tcc1Mc1,
    Tcc1Mc2,
    Tcc1Mc3,
    Tcc2Ovf,
    Tcc2Mc0,
    Tcc2Mc1,
    Tcc2Mc2,
    Tcc3Ovf,
    Tcc3Mc0,
    Tcc3Mc1,
    Tcc4Ovf,
    Tcc4Mc0,
    Tcc4Mc1,
    Tc0Ovf,
    Tc0Mc0,
    Tc0Mc1,
    Tc1Ovf,
    Tc1Mc0,
    Tc1Mc1,
    Tc2Ovf,
    Tc2Mc0,
    Tc2Mc1,
    Tc3Ovf,
    Tc3Mc0,
    Tc3Mc1,
    Tc4Ovf,
    Tc4Mc0,
    Tc4Mc1,
    Tc5Ovf,
    Tc5Mc0,
    Tc5Mc1,
    Tc6Ovf,
    Tc6Mc0,
    Tc6Mc1,
    Tc7Ovf,
    Tc7Mc0,
    Tc7Mc1,
    Adc0ResRdy,
    Adc0Seq,
    Adc1ResRdy,
    Adc1Seq,
    Dac0Empty,
    Dac1Empty,
    Dac0ResRdy,
    Dac1ResRdy,
    I2sRx0,
    I2sRx1,
    I2sTx0,
    I2sTx1,
    PccRx,
    AesWr,
    AesRd,
    QspiRx,
    QspiTx,
}

impl From<Variant<u8, TRIGSRC_A>> for TriggerSource {
    fn from(value: Variant<u8, TRIGSRC_A>) -> TriggerSource {
        use self::TriggerSource::*;
        use self::Variant::*;
        match value {
            Val(TRIGSRC_A::DISABLE) => Disable,
            Res(1) => RtcTimestamp,
            Res(2) => DsuDcc0,
            Res(3) => DsuDcc1,
            Res(4) => Sercom0Rx,
            Res(5) => Sercom0Tx,
            Res(6) => Sercom1Rx,
            Res(7) => Sercom1Tx,
            Res(8) => Sercom2Rx,
            Res(9) => Sercom3Tx,
            Res(10) => Sercom4Rx,
            Res(11) => Sercom4Tx,
            Res(12) => Sercom5Rx,
            Res(13) => Sercom5Tx,
            Res(14) => Sercom6Rx,
            Res(15) => Sercom6Tx,
            Res(16) => Sercom7Rx,
            Res(17) => Sercom7Tx,
            Res(18) => Can0Debug,
            Res(19) => Can1Debug,
            Res(20) => Tcc0Ovf,
            Res(21) => Tcc0Mc0,
            Res(22) => Tcc0Mc1,
            Res(23) => Tcc0Mc2,
            Res(24) => Tcc0Mc3,
            Res(25) => Tcc0Mc4,
            Res(26) => Tcc0Mc5,
            Res(27) => Tcc1Ovf,
            Res(28) => Tcc1Mc0,
            Res(29) => Tcc1Mc1,
            Res(30) => Tcc1Mc2,
            Res(31) => Tcc1Mc3,
            Res(32) => Tcc2Ovf,
            Res(33) => Tcc2Mc0,
            Res(34) => Tcc2Mc1,
            Res(35) => Tcc2Mc2,
            Res(36) => Tcc3Ovf,
            Res(37) => Tcc3Mc0,
            Res(38) => Tcc3Mc1,
            Res(39) => Tcc4Ovf,
            Res(40) => Tcc4Mc0,
            Res(41) => Tcc4Mc1,
            Res(42) => Tc0Ovf,
            Res(43) => Tc0Mc0,
            Res(44) => Tc0Mc1,
            Res(45) => Tc1Ovf,
            Res(46) => Tc1Mc0,
            Res(47) => Tc1Mc1,
            Res(48) => Tc2Ovf,
            Res(49) => Tc2Mc0,
            Res(50) => Tc2Mc1,
            Res(51) => Tc3Ovf,
            Res(52) => Tc3Mc0,
            Res(53) => Tc3Mc1,
            Res(54) => Tc4Ovf,
            Res(55) => Tc4Mc0,
            Res(56) => Tc4Mc1,
            Res(57) => Tc5Ovf,
            Res(58) => Tc5Mc0,
            Res(59) => Tc5Mc1,
            Res(60) => Tc6Ovf,
            Res(61) => Tc6Mc0,
            Res(62) => Tc6Mc1,
            Res(63) => Tc7Ovf,
            Res(64) => Tc7Mc0,
            Res(65) => Tc7Mc1,
            Res(66) => Adc0ResRdy,
            Res(67) => Adc0Seq,
            Res(68) => Adc1ResRdy,
            Res(69) => Adc1Seq,
            Res(70) => Dac0Empty,
            Res(71) => Dac1Empty,
            Res(72) => Dac0ResRdy,
            Res(73) => Dac1ResRdy,
            Res(74) => I2sRx0,
            Res(75) => I2sRx1,
            Res(76) => I2sTx0,
            Res(77) => I2sTx1,
            Res(78) => PccRx,
            Res(79) => AesWr,
            Res(80) => AesRd,
            Res(81) => QspiRx,
            Res(82) => QspiTx,
            Res(_) => Disable,
        }
    }
}

/// Number of beats before destination writes occur.
#[cfg(feature = "samd5x")]
pub enum FifoThreshold {
    OneBeat = 0,
    TwoBeats,
    FourBeats,
    EightBeats,
}

#[cfg(feature = "samd5x")]
impl From<THRESHOLD_A> for FifoThreshold {
    fn from(value: THRESHOLD_A) -> FifoThreshold {
        use self::FifoThreshold::*;
        match value {
            THRESHOLD_A::_1BEAT => OneBeat,
            THRESHOLD_A::_2BEATS => TwoBeats,
            THRESHOLD_A::_4BEATS => FourBeats,
            THRESHOLD_A::_8BEATS => EightBeats,
        }
    }
}

#[derive(SmartDefault)]
/// When EVSYS events should be output.
pub enum EventOutput {
    #[default]
    Disable = 0,
    Block,
    Beat = 3,
}

impl EventOutput {
    pub fn from(value: u16) -> EventOutput {
        use self::EventOutput::*;
        match value {
            0 => Disable,
            1 => Block,
            3 => Beat,
            _ => unreachable!()
        }
    }
}

/// Define what happens when a block transfer completes.
#[derive(SmartDefault)]
pub enum BlockAction {
    /// Channel will be disabled if this is the last block transfer.
    #[default]
    NoAct = 0,
    /// Block interrupt will be generated, plus the action of NoAct.
    Int,
    /// Channel will be suspended.
    Suspend,
    /// Channel will be suspended and block interrupt will be generated.
    Both,
}

impl BlockAction {
    pub fn from(value: u16) -> BlockAction {
        use self::BlockAction::*;
        match value {
            0 => NoAct,
            1 => Int,
            2 => Suspend,
            3 => Both,
            _ => unreachable!()
        }
    }
}

/// Size of a DMA beat transfer memory access.
pub enum BeatSize {
    Byte = 0,
    HWord,
    Word
}

impl BeatSize {
    pub fn from(value: u16) -> BeatSize {
        use self::BeatSize::*;
        match value {
            0 => Byte,
            1 => HWord,
            2 => Word,
            _ => unreachable!()
        }
    }
}

/// Size of the address advancement step.
#[derive(SmartDefault)]
pub enum StepSize {
    #[default]
    X1 = 0,
    X2,
    X4,
    X8,
    X16,
    X32,
    X64,
    X128,
}

impl StepSize {
    pub fn from(value: u16) -> StepSize {
        use self::StepSize::*;
        match value {
            0 => X1,
            1 => X2,
            2 => X4,
            3 => X8,
            4 => X16,
            5 => X32,
            6 => X64,
            7 => X128,
            _ => unreachable!()
        }
    }
}
