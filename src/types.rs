#![allow(missing_docs)]
use smart_default::SmartDefault;

use crate::target_device::generic::Variant;
#[cfg(feature = "samd5x")]
use crate::target_device::dmac::channel::chctrla::{TRIGACT_A, BURSTLEN_A, THRESHOLD_A, TRIGSRC_A};
#[cfg(feature = "samd5x")]
use crate::target_device::dmac::channel::chprilvl::PRILVL_A;
#[cfg(feature = "samd5x")]
use crate::target_device::dmac::prictrl0::{QOS0_A, QOS1_A, QOS2_A, QOS3_A};
#[cfg(feature = "samd21")]
use crate::target_device::dmac::chctrlb::{TRIGACT_A, TRIGSRC_A, LVL_A as PRILVL_A};
#[cfg(feature = "samd21")]
use crate::target_device::dmac::qosctrl::{DQOS_A, FQOS_A, WRBQOS_A};

bitflags! {
    /// A bitfield of possible channels.
    pub struct Channels: u32 {
        const CHAN0 = 1 << 0;
        const CHAN1 = 1 << 1;
        const CHAN2 = 1 << 2;
        const CHAN3 = 1 << 3;
        const CHAN4 = 1 << 4;
        const CHAN5 = 1 << 5;
        const CHAN6 = 1 << 6;
        const CHAN7 = 1 << 7;
        const CHAN8 = 1 << 8;
        const CHAN9 = 1 << 9;
        const CHAN10 = 1 << 10;
        const CHAN11 = 1 << 11;
        #[cfg(feature = "samd5x")]
        const CHAN12 = 1 << 12;
        #[cfg(feature = "samd5x")]
        const CHAN13 = 1 << 13;
        #[cfg(feature = "samd5x")]
        const CHAN14 = 1 << 14;
        #[cfg(feature = "samd5x")]
        const CHAN15 = 1 << 15;
        #[cfg(feature = "samd5x")]
        const CHAN16 = 1 << 16;
        #[cfg(feature = "samd5x")]
        const CHAN17 = 1 << 17;
        #[cfg(feature = "samd5x")]
        const CHAN18 = 1 << 18;
        #[cfg(feature = "samd5x")]
        const CHAN19 = 1 << 19;
        #[cfg(feature = "samd5x")]
        const CHAN20 = 1 << 20;
        #[cfg(feature = "samd5x")]
        const CHAN21 = 1 << 21;
        #[cfg(feature = "samd5x")]
        const CHAN22 = 1 << 22;
        #[cfg(feature = "samd5x")]
        const CHAN23 = 1 << 23;
        #[cfg(feature = "samd5x")]
        const CHAN24 = 1 << 24;
        #[cfg(feature = "samd5x")]
        const CHAN25 = 1 << 25;
        #[cfg(feature = "samd5x")]
        const CHAN26 = 1 << 26;
        #[cfg(feature = "samd5x")]
        const CHAN27 = 1 << 27;
        #[cfg(feature = "samd5x")]
        const CHAN28 = 1 << 28;
        #[cfg(feature = "samd5x")]
        const CHAN29 = 1 << 29;
        #[cfg(feature = "samd5x")]
        const CHAN30 = 1 << 30;
        #[cfg(feature = "samd5x")]
        const CHAN31 = 1 << 31;
    }
}

bitflags! {
    /// A bitfield to represent channel interrupt flags.
    pub struct Interrupts: u8 {
        const TERR = 0x1;
        const TCMPL = 0x2;
        const SUSP = 0x4;
    }
}

/// The status of a channel.
pub enum Status {
    Busy,
    Pending,
    FetchError,
    #[cfg(feature = "samd5x")]
    CRCError
}

/// Priority level of a channel.
pub enum Priority {
    Level0 = 0,
    Level1,
    Level2,
    Level3,
}

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

/// Quality of Service guarantee for the DMA system.
pub enum QoS {
    Disable = 0,
    Low,
    Medium,
    Critical,
}

#[cfg(feature = "samd5x")]
impl From<QOS0_A> for QoS {
    fn from(val: QOS0_A) -> QoS {
        use self::QoS::*;
        use self::QOS0_A::*;
        match val {
            REGULAR => Disable,
            SHORTAGE => Low,
            SENSITIVE => Medium,
            CRITICAL => Critical
        }
    }
}

#[cfg(feature = "samd5x")]
impl From<QOS1_A> for QoS {
    fn from(val: QOS1_A) -> QoS {
        use self::QoS::*;
        use self::QOS1_A::*;
        match val {
            REGULAR => Disable,
            SHORTAGE => Low,
            SENSITIVE => Medium,
            CRITICAL => Critical
        }
    }
}

#[cfg(feature = "samd5x")]
impl From<QOS2_A> for QoS {
    fn from(val: QOS2_A) -> QoS {
        use self::QoS::*;
        use self::QOS2_A::*;
        match val {
            REGULAR => Disable,
            SHORTAGE => Low,
            SENSITIVE => Medium,
            CRITICAL => Critical
        }
    }
}

#[cfg(feature = "samd5x")]
impl From<QOS3_A> for QoS {
    fn from(val: QOS3_A) -> QoS {
        use self::QoS::*;
        use self::QOS3_A::*;
        match val {
            REGULAR => Disable,
            SHORTAGE => Low,
            SENSITIVE => Medium,
            CRITICAL => Critical
        }
    }
}

#[cfg(feature = "samd21")]
impl From<DQOS_A> for QoS {
    fn from(val: DQOS_A) -> QoS {
        use self::QoS::*;
        use self::DQOS_A::*;
        match val {
            DISABLE => Disable,
            LOW => Low,
            MEDIUM => Medium,
            HIGH => Critical
        }
    }
}

#[cfg(feature = "samd21")]
impl From<FQOS_A> for QoS {
    fn from(val: FQOS_A) -> QoS {
        use self::QoS::*;
        use self::FQOS_A::*;
        match val {
            DISABLE => Disable,
            LOW => Low,
            MEDIUM => Medium,
            HIGH => Critical
        }
    }
}

#[cfg(feature = "samd21")]
impl From<WRBQOS_A> for QoS {
    fn from(val: WRBQOS_A) -> QoS {
        use self::QoS::*;
        use self::WRBQOS_A::*;
        match val {
            DISABLE => Disable,
            LOW => Low,
            MEDIUM => Medium,
            HIGH => Critical
        }
    }
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
#[cfg(feature = "samd5x")]
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
    Sercom2Tx,
    Sercom3Rx,
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

/// Trigger source for a channel.
#[cfg(feature = "samd21")]
pub enum TriggerSource {
    Disable = 0,
    Sercom0Rx,
    Sercom0Tx,
    Sercom1Rx,
    Sercom1Tx,
    Sercom2Rx,
    Sercom2Tx,
    Sercom3Rx,
    Sercom3Tx,
    Sercom4Rx,
    Sercom4Tx,
    Sercom5Rx,
    Sercom5Tx,
    Tcc0Ovf,
    Tcc0Mc0,
    Tcc0Mc1,
    Tcc0Mc2,
    Tcc0Mc3,
    Tcc1Ovf,
    Tcc1Mc0,
    Tcc1Mc1,
    Tcc2Ovf,
    Tcc2Mc0,
    Tcc2Mc1,
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
    AdcResRdy,
    DacEmpty,
    I2sRx0,
    I2sRx1,
    I2sTx0,
    I2sTx1,
    Tcc3Ovf,
    Tcc3Mc0,
    Tcc3Mc1,
    Tcc3Mc2,
    Tcc3Mc3,
}

impl From<Variant<u8, TRIGSRC_A>> for TriggerSource {
    #[cfg(feature = "samd5x")]
    fn from(value: Variant<u8, TRIGSRC_A>) -> TriggerSource {
        use self::TriggerSource::*;
        use self::Variant::*;
        match value {
            Val(TRIGSRC_A::DISABLE) => Disable,
            Val(TRIGSRC_A::RTC_TIMESTAMP) => RtcTimestamp,
            Val(TRIGSRC_A::DSU_DCC0) => DsuDcc0,
            Val(TRIGSRC_A::DSU_DCC1) => DsuDcc1,
            Val(TRIGSRC_A::SERCOM0_RX) => Sercom0Rx,
            Val(TRIGSRC_A::SERCOM0_TX) => Sercom0Tx,
            Val(TRIGSRC_A::SERCOM1_RX) => Sercom1Rx,
            Val(TRIGSRC_A::SERCOM1_TX) => Sercom1Tx,
            Val(TRIGSRC_A::SERCOM2_RX) => Sercom2Rx,
            Val(TRIGSRC_A::SERCOM2_TX) => Sercom2Tx,
            Val(TRIGSRC_A::SERCOM3_RX) => Sercom3Rx,
            Val(TRIGSRC_A::SERCOM3_TX) => Sercom3Tx,
            Val(TRIGSRC_A::SERCOM4_RX) => Sercom4Rx,
            Val(TRIGSRC_A::SERCOM4_TX) => Sercom4Tx,
            Val(TRIGSRC_A::SERCOM5_RX) => Sercom5Rx,
            Val(TRIGSRC_A::SERCOM5_TX) => Sercom5Tx,
            Val(TRIGSRC_A::SERCOM6_RX) => Sercom6Rx,
            Val(TRIGSRC_A::SERCOM6_TX) => Sercom6Tx,
            Val(TRIGSRC_A::SERCOM7_RX) => Sercom7Rx,
            Val(TRIGSRC_A::SERCOM7_TX) => Sercom7Tx,
            Val(TRIGSRC_A::CAN0_DEBUG) => Can0Debug,
            Val(TRIGSRC_A::CAN1_DEBUG) => Can1Debug,
            Val(TRIGSRC_A::TCC0_OVF) => Tcc0Ovf,
            Val(TRIGSRC_A::TCC0_MC_0) => Tcc0Mc0,
            Val(TRIGSRC_A::TCC0_MC_1) => Tcc0Mc1,
            Val(TRIGSRC_A::TCC0_MC_2) => Tcc0Mc2,
            Val(TRIGSRC_A::TCC0_MC_3) => Tcc0Mc3,
            Val(TRIGSRC_A::TCC0_MC_4) => Tcc0Mc4,
            Val(TRIGSRC_A::TCC0_MC_5) => Tcc0Mc5,
            Val(TRIGSRC_A::TCC1_OVF) => Tcc1Ovf,
            Val(TRIGSRC_A::TCC1_MC_0) => Tcc1Mc0,
            Val(TRIGSRC_A::TCC1_MC_1) => Tcc1Mc1,
            Val(TRIGSRC_A::TCC1_MC_2) => Tcc1Mc2,
            Val(TRIGSRC_A::TCC1_MC_3) => Tcc1Mc3,
            Val(TRIGSRC_A::TCC2_OVF) => Tcc2Ovf,
            Val(TRIGSRC_A::TCC2_MC_0) => Tcc2Mc0,
            Val(TRIGSRC_A::TCC2_MC_1) => Tcc2Mc1,
            Val(TRIGSRC_A::TCC2_MC_2) => Tcc2Mc2,
            Val(TRIGSRC_A::TCC3_OVF) => Tcc3Ovf,
            Val(TRIGSRC_A::TCC3_MC_0) => Tcc3Mc0,
            Val(TRIGSRC_A::TCC3_MC_1) => Tcc3Mc1,
            Val(TRIGSRC_A::TCC4_OVF) => Tcc4Ovf,
            Val(TRIGSRC_A::TCC4_MC_0) => Tcc4Mc0,
            Val(TRIGSRC_A::TCC4_MC_1) => Tcc4Mc1,
            Val(TRIGSRC_A::TC0_OVF) => Tc0Ovf,
            Val(TRIGSRC_A::TC0_MC_0) => Tc0Mc0,
            Val(TRIGSRC_A::TC0_MC_1) => Tc0Mc1,
            Val(TRIGSRC_A::TC1_OVF) => Tc1Ovf,
            Val(TRIGSRC_A::TC1_MC_0) => Tc1Mc0,
            Val(TRIGSRC_A::TC1_MC_1) => Tc1Mc1,
            Val(TRIGSRC_A::TC2_OVF) => Tc2Ovf,
            Val(TRIGSRC_A::TC2_MC_0) => Tc2Mc0,
            Val(TRIGSRC_A::TC2_MC_1) => Tc2Mc1,
            Val(TRIGSRC_A::TC3_OVF) => Tc3Ovf,
            Val(TRIGSRC_A::TC3_MC_0) => Tc3Mc0,
            Val(TRIGSRC_A::TC3_MC_1) => Tc3Mc1,
            Val(TRIGSRC_A::TC4_OVF) => Tc4Ovf,
            Val(TRIGSRC_A::TC4_MC_0) => Tc4Mc0,
            Val(TRIGSRC_A::TC4_MC_1) => Tc4Mc1,
            Val(TRIGSRC_A::TC5_OVF) => Tc5Ovf,
            Val(TRIGSRC_A::TC5_MC_0) => Tc5Mc0,
            Val(TRIGSRC_A::TC5_MC_1) => Tc5Mc1,
            Val(TRIGSRC_A::TC6_OVF) => Tc6Ovf,
            Val(TRIGSRC_A::TC6_MC_0) => Tc6Mc0,
            Val(TRIGSRC_A::TC6_MC_1) => Tc6Mc1,
            Val(TRIGSRC_A::TC7_OVF) => Tc7Ovf,
            Val(TRIGSRC_A::TC7_MC_0) => Tc7Mc0,
            Val(TRIGSRC_A::TC7_MC_1) => Tc7Mc1,
            Val(TRIGSRC_A::ADC0_RESRDY) => Adc0ResRdy,
            Val(TRIGSRC_A::ADC0_SEQ) => Adc0Seq,
            Val(TRIGSRC_A::ADC1_RESRDY) => Adc1ResRdy,
            Val(TRIGSRC_A::ADC1_SEQ) => Adc1Seq,
            Val(TRIGSRC_A::DAC_EMPTY_0) => Dac0Empty,
            Val(TRIGSRC_A::DAC_EMPTY_1) => Dac1Empty,
            Val(TRIGSRC_A::DAC_RESRDY_0) => Dac0ResRdy,
            Val(TRIGSRC_A::DAC_RESRDY_1) => Dac1ResRdy,
            Val(TRIGSRC_A::I2S_RX_0) => I2sRx0,
            Val(TRIGSRC_A::I2S_RX_1) => I2sRx1,
            Val(TRIGSRC_A::I2S_TX_0) => I2sTx0,
            Val(TRIGSRC_A::I2S_TX_1) => I2sTx1,
            Val(TRIGSRC_A::PCC_RX) => PccRx,
            Val(TRIGSRC_A::AES_WR) => AesWr,
            Val(TRIGSRC_A::AES_RD) => AesRd,
            Val(TRIGSRC_A::QSPI_RX) => QspiRx,
            Val(TRIGSRC_A::QSPI_TX) => QspiTx,
            Res(_) => Disable,
        }
    }

    #[cfg(feature = "samd21")]
    fn from(value: Variant<u8, TRIGSRC_A>) -> TriggerSource {
        use self::TriggerSource::*;
        use self::Variant::*;
        match value {
            Val(TRIGSRC_A::DISABLE) => Disable,
            Val(TRIGSRC_A::SERCOM0_RX) => Sercom0Rx,
            Val(TRIGSRC_A::SERCOM0_TX) => Sercom0Tx,
            Val(TRIGSRC_A::SERCOM1_RX) => Sercom1Rx,
            Val(TRIGSRC_A::SERCOM1_TX) => Sercom1Tx,
            Val(TRIGSRC_A::SERCOM2_RX) => Sercom2Rx,
            Val(TRIGSRC_A::SERCOM2_TX) => Sercom2Tx,
            Val(TRIGSRC_A::SERCOM3_RX) => Sercom3Rx,
            Val(TRIGSRC_A::SERCOM3_TX) => Sercom3Tx,
            Val(TRIGSRC_A::SERCOM4_RX) => Sercom4Rx,
            Val(TRIGSRC_A::SERCOM4_TX) => Sercom4Tx,
            Val(TRIGSRC_A::SERCOM5_RX) => Sercom5Rx,
            Val(TRIGSRC_A::SERCOM5_TX) => Sercom5Tx,
            Val(TRIGSRC_A::TCC0_OVF) => Tcc0Ovf,
            Val(TRIGSRC_A::TCC0_MC0) => Tcc0Mc0,
            Val(TRIGSRC_A::TCC0_MC1) => Tcc0Mc1,
            Val(TRIGSRC_A::TCC0_MC2) => Tcc0Mc2,
            Val(TRIGSRC_A::TCC0_MC3) => Tcc0Mc3,
            Val(TRIGSRC_A::TCC1_OVF) => Tcc1Ovf,
            Val(TRIGSRC_A::TCC1_MC0) => Tcc1Mc0,
            Val(TRIGSRC_A::TCC1_MC1) => Tcc1Mc1,
            Val(TRIGSRC_A::TCC2_OVF) => Tcc2Ovf,
            Val(TRIGSRC_A::TCC2_MC0) => Tcc2Mc0,
            Val(TRIGSRC_A::TCC2_MC1) => Tcc2Mc1,
            Val(TRIGSRC_A::TC3_OVF) => Tc3Ovf,
            Val(TRIGSRC_A::TC3_MC0) => Tc3Mc0,
            Val(TRIGSRC_A::TC3_MC1) => Tc3Mc1,
            Val(TRIGSRC_A::TC4_OVF) => Tc4Ovf,
            Val(TRIGSRC_A::TC4_MC0) => Tc4Mc0,
            Val(TRIGSRC_A::TC4_MC1) => Tc4Mc1,
            Val(TRIGSRC_A::TC5_OVF) => Tc5Ovf,
            Val(TRIGSRC_A::TC5_MC0) => Tc5Mc0,
            Val(TRIGSRC_A::TC5_MC1) => Tc5Mc1,
            Val(TRIGSRC_A::TC6_OVF) => Tc6Ovf,
            Val(TRIGSRC_A::TC6_MC0) => Tc6Mc0,
            Val(TRIGSRC_A::TC6_MC1) => Tc6Mc1,
            Val(TRIGSRC_A::TC7_OVF) => Tc7Ovf,
            Val(TRIGSRC_A::TC7_MC0) => Tc7Mc0,
            Val(TRIGSRC_A::TC7_MC1) => Tc7Mc1,
            Val(TRIGSRC_A::ADC_RESRDY) => AdcResRdy,
            Val(TRIGSRC_A::DAC_EMPTY) => DacEmpty,
            Val(TRIGSRC_A::I2S_RX_0) => I2sRx0,
            Val(TRIGSRC_A::I2S_RX_1) => I2sRx1,
            Val(TRIGSRC_A::I2S_TX_0) => I2sTx0,
            Val(TRIGSRC_A::I2S_TX_1) => I2sTx1,
            Val(TRIGSRC_A::TCC3_OVF) => Tcc3Ovf,
            Val(TRIGSRC_A::TCC3_MC0) => Tcc3Mc0,
            Val(TRIGSRC_A::TCC3_MC1) => Tcc3Mc1,
            Val(TRIGSRC_A::TCC3_MC2) => Tcc3Mc2,
            Val(TRIGSRC_A::TCC3_MC3) => Tcc3Mc3,
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

/// When EVSYS events should be output.
#[derive(SmartDefault)]
pub enum EventOutput {
    #[default]
    Disable = 0,
    Block,
    Beat = 3,
}

impl EventOutput {
    pub(crate) fn from(value: u16) -> EventOutput {
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
    pub(crate) fn from(value: u16) -> BlockAction {
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
    pub(crate) fn from(value: u16) -> BeatSize {
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
    pub(crate) fn from(value: u16) -> StepSize {
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
