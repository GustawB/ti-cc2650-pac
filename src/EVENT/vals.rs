#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AUXSEL0_EV(u8);
impl AUXSEL0_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
    #[doc = "GPT2A interrupt event, controlled by GPT2:TAMR."]
    pub const GPT2A: Self = Self(0x0c);
    #[doc = "GPT2B interrupt event, controlled by GPT2:TBMR."]
    pub const GPT2B: Self = Self(0x0d);
    #[doc = "GPT3A interrupt event, controlled by GPT3:TAMR."]
    pub const GPT3A: Self = Self(0x0e);
    #[doc = "GPT3B interrupt event, controlled by GPT3:TBMR."]
    pub const GPT3B: Self = Self(0x0f);
    #[doc = "GPT0A interrupt event, controlled by GPT0:TAMR."]
    pub const GPT0A: Self = Self(0x10);
    #[doc = "GPT0B interrupt event, controlled by GPT0:TBMR."]
    pub const GPT0B: Self = Self(0x11);
    #[doc = "GPT1A interrupt event, controlled by GPT1:TAMR."]
    pub const GPT1A: Self = Self(0x12);
    #[doc = "GPT1B interrupt event, controlled by GPT1:TBMR."]
    pub const GPT1B: Self = Self(0x13);
    #[doc = "Always asserted."]
    pub const ALWAYS_ACTIVE: Self = Self(0x79);
}
impl AUXSEL0_EV {
    pub const fn from_bits(val: u8) -> AUXSEL0_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for AUXSEL0_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            0x0c => f.write_str("GPT2A"),
            0x0d => f.write_str("GPT2B"),
            0x0e => f.write_str("GPT3A"),
            0x0f => f.write_str("GPT3B"),
            0x10 => f.write_str("GPT0A"),
            0x11 => f.write_str("GPT0B"),
            0x12 => f.write_str("GPT1A"),
            0x13 => f.write_str("GPT1B"),
            0x79 => f.write_str("ALWAYS_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUXSEL0_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            0x0c => defmt::write!(f, "GPT2A"),
            0x0d => defmt::write!(f, "GPT2B"),
            0x0e => defmt::write!(f, "GPT3A"),
            0x0f => defmt::write!(f, "GPT3B"),
            0x10 => defmt::write!(f, "GPT0A"),
            0x11 => defmt::write!(f, "GPT0B"),
            0x12 => defmt::write!(f, "GPT1A"),
            0x13 => defmt::write!(f, "GPT1B"),
            0x79 => defmt::write!(f, "ALWAYS_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for AUXSEL0_EV {
    #[inline(always)]
    fn from(val: u8) -> AUXSEL0_EV {
        AUXSEL0_EV::from_bits(val)
    }
}
impl From<AUXSEL0_EV> for u8 {
    #[inline(always)]
    fn from(val: AUXSEL0_EV) -> u8 {
        AUXSEL0_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CM3NMISEL0_EV(u8);
impl CM3NMISEL0_EV {
    #[doc = "Watchdog non maskable interrupt event, controlled by WDT:CTL.INTTYPE."]
    pub const WDT_NMI: Self = Self(0x63);
}
impl CM3NMISEL0_EV {
    pub const fn from_bits(val: u8) -> CM3NMISEL0_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CM3NMISEL0_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x63 => f.write_str("WDT_NMI"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CM3NMISEL0_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x63 => defmt::write!(f, "WDT_NMI"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CM3NMISEL0_EV {
    #[inline(always)]
    fn from(val: u8) -> CM3NMISEL0_EV {
        CM3NMISEL0_EV::from_bits(val)
    }
}
impl From<CM3NMISEL0_EV> for u8 {
    #[inline(always)]
    fn from(val: CM3NMISEL0_EV) -> u8 {
        CM3NMISEL0_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL0_EV(u8);
impl CPUIRQSEL0_EV {
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings."]
    pub const AON_GPIO_EDGE: Self = Self(0x04);
}
impl CPUIRQSEL0_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL0_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL0_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x04 => f.write_str("AON_GPIO_EDGE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL0_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x04 => defmt::write!(f, "AON_GPIO_EDGE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL0_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL0_EV {
        CPUIRQSEL0_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL0_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL0_EV) -> u8 {
        CPUIRQSEL0_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL10_EV(u8);
impl CPUIRQSEL10_EV {
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG."]
    pub const RFC_HW_COMB: Self = Self(0x1a);
}
impl CPUIRQSEL10_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL10_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL10_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x1a => f.write_str("RFC_HW_COMB"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL10_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x1a => defmt::write!(f, "RFC_HW_COMB"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL10_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL10_EV {
        CPUIRQSEL10_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL10_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL10_EV) -> u8 {
        CPUIRQSEL10_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL11_EV(u8);
impl CPUIRQSEL11_EV {
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG."]
    pub const RFC_CMD_ACK: Self = Self(0x19);
}
impl CPUIRQSEL11_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL11_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL11_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x19 => f.write_str("RFC_CMD_ACK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL11_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x19 => defmt::write!(f, "RFC_CMD_ACK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL11_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL11_EV {
        CPUIRQSEL11_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL11_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL11_EV) -> u8 {
        CPUIRQSEL11_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL12_EV(u8);
impl CPUIRQSEL12_EV {
    #[doc = "Interrupt event from I2S."]
    pub const I2S_IRQ: Self = Self(0x08);
}
impl CPUIRQSEL12_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL12_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL12_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x08 => f.write_str("I2S_IRQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL12_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x08 => defmt::write!(f, "I2S_IRQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL12_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL12_EV {
        CPUIRQSEL12_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL12_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL12_EV) -> u8 {
        CPUIRQSEL12_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL13_EV(u8);
impl CPUIRQSEL13_EV {
    #[doc = "AUX software event 1, triggered by AUX_EVCTL:SWEVSET.SWEV1, also available as AUX_EVENT2 AON wake up event. MCU domain wakeup control AON_EVENT:MCUWUSEL AUX domain wakeup control AON_EVENT:AUXWUSEL."]
    pub const AUX_SWEV1: Self = Self(0x1d);
}
impl CPUIRQSEL13_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL13_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL13_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x1d => f.write_str("AUX_SWEV1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL13_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x1d => defmt::write!(f, "AUX_SWEV1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL13_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL13_EV {
        CPUIRQSEL13_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL13_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL13_EV) -> u8 {
        CPUIRQSEL13_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL14_EV(u8);
impl CPUIRQSEL14_EV {
    #[doc = "Watchdog interrupt event, controlled by WDT:CTL.INTEN."]
    pub const WDT_IRQ: Self = Self(0x18);
}
impl CPUIRQSEL14_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL14_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL14_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x18 => f.write_str("WDT_IRQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL14_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x18 => defmt::write!(f, "WDT_IRQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL14_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL14_EV {
        CPUIRQSEL14_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL14_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL14_EV) -> u8 {
        CPUIRQSEL14_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL15_EV(u8);
impl CPUIRQSEL15_EV {
    #[doc = "GPT0A interrupt event, controlled by GPT0:TAMR."]
    pub const GPT0A: Self = Self(0x10);
}
impl CPUIRQSEL15_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL15_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL15_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x10 => f.write_str("GPT0A"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL15_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x10 => defmt::write!(f, "GPT0A"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL15_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL15_EV {
        CPUIRQSEL15_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL15_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL15_EV) -> u8 {
        CPUIRQSEL15_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL16_EV(u8);
impl CPUIRQSEL16_EV {
    #[doc = "GPT0B interrupt event, controlled by GPT0:TBMR."]
    pub const GPT0B: Self = Self(0x11);
}
impl CPUIRQSEL16_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL16_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL16_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x11 => f.write_str("GPT0B"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL16_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x11 => defmt::write!(f, "GPT0B"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL16_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL16_EV {
        CPUIRQSEL16_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL16_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL16_EV) -> u8 {
        CPUIRQSEL16_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL17_EV(u8);
impl CPUIRQSEL17_EV {
    #[doc = "GPT1A interrupt event, controlled by GPT1:TAMR."]
    pub const GPT1A: Self = Self(0x12);
}
impl CPUIRQSEL17_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL17_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL17_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x12 => f.write_str("GPT1A"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL17_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x12 => defmt::write!(f, "GPT1A"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL17_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL17_EV {
        CPUIRQSEL17_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL17_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL17_EV) -> u8 {
        CPUIRQSEL17_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL18_EV(u8);
impl CPUIRQSEL18_EV {
    #[doc = "GPT1B interrupt event, controlled by GPT1:TBMR."]
    pub const GPT1B: Self = Self(0x13);
}
impl CPUIRQSEL18_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL18_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL18_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x13 => f.write_str("GPT1B"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL18_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x13 => defmt::write!(f, "GPT1B"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL18_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL18_EV {
        CPUIRQSEL18_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL18_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL18_EV) -> u8 {
        CPUIRQSEL18_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL19_EV(u8);
impl CPUIRQSEL19_EV {
    #[doc = "GPT2A interrupt event, controlled by GPT2:TAMR."]
    pub const GPT2A: Self = Self(0x0c);
}
impl CPUIRQSEL19_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL19_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL19_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0c => f.write_str("GPT2A"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL19_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0c => defmt::write!(f, "GPT2A"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL19_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL19_EV {
        CPUIRQSEL19_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL19_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL19_EV) -> u8 {
        CPUIRQSEL19_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL1_EV(u8);
impl CPUIRQSEL1_EV {
    #[doc = "Interrupt event from I2C."]
    pub const I2C_IRQ: Self = Self(0x09);
}
impl CPUIRQSEL1_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL1_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL1_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x09 => f.write_str("I2C_IRQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL1_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x09 => defmt::write!(f, "I2C_IRQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL1_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL1_EV {
        CPUIRQSEL1_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL1_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL1_EV) -> u8 {
        CPUIRQSEL1_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL20_EV(u8);
impl CPUIRQSEL20_EV {
    #[doc = "GPT2B interrupt event, controlled by GPT2:TBMR."]
    pub const GPT2B: Self = Self(0x0d);
}
impl CPUIRQSEL20_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL20_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL20_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0d => f.write_str("GPT2B"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL20_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0d => defmt::write!(f, "GPT2B"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL20_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL20_EV {
        CPUIRQSEL20_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL20_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL20_EV) -> u8 {
        CPUIRQSEL20_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL21_EV(u8);
impl CPUIRQSEL21_EV {
    #[doc = "GPT3A interrupt event, controlled by GPT3:TAMR."]
    pub const GPT3A: Self = Self(0x0e);
}
impl CPUIRQSEL21_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL21_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL21_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0e => f.write_str("GPT3A"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL21_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0e => defmt::write!(f, "GPT3A"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL21_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL21_EV {
        CPUIRQSEL21_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL21_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL21_EV) -> u8 {
        CPUIRQSEL21_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL22_EV(u8);
impl CPUIRQSEL22_EV {
    #[doc = "GPT3B interrupt event, controlled by GPT3:TBMR."]
    pub const GPT3B: Self = Self(0x0f);
}
impl CPUIRQSEL22_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL22_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL22_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0f => f.write_str("GPT3B"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL22_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0f => defmt::write!(f, "GPT3B"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL22_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL22_EV {
        CPUIRQSEL22_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL22_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL22_EV) -> u8 {
        CPUIRQSEL22_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL23_EV(u8);
impl CPUIRQSEL23_EV {
    #[doc = "CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL."]
    pub const CRYPTO_RESULT_AVAIL_IRQ: Self = Self(0x5d);
}
impl CPUIRQSEL23_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL23_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL23_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x5d => f.write_str("CRYPTO_RESULT_AVAIL_IRQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL23_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x5d => defmt::write!(f, "CRYPTO_RESULT_AVAIL_IRQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL23_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL23_EV {
        CPUIRQSEL23_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL23_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL23_EV) -> u8 {
        CPUIRQSEL23_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL24_EV(u8);
impl CPUIRQSEL24_EV {
    #[doc = "Combined DMA done, corresponding flags are here UDMA0:REQDONE."]
    pub const DMA_DONE_COMB: Self = Self(0x27);
}
impl CPUIRQSEL24_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL24_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL24_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x27 => f.write_str("DMA_DONE_COMB"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL24_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x27 => defmt::write!(f, "DMA_DONE_COMB"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL24_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL24_EV {
        CPUIRQSEL24_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL24_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL24_EV) -> u8 {
        CPUIRQSEL24_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL25_EV(u8);
impl CPUIRQSEL25_EV {
    #[doc = "DMA bus error, corresponds to UDMA0:ERROR.STATUS."]
    pub const DMA_ERR: Self = Self(0x26);
}
impl CPUIRQSEL25_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL25_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL25_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x26 => f.write_str("DMA_ERR"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL25_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x26 => defmt::write!(f, "DMA_ERR"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL25_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL25_EV {
        CPUIRQSEL25_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL25_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL25_EV) -> u8 {
        CPUIRQSEL25_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL26_EV(u8);
impl CPUIRQSEL26_EV {
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT."]
    pub const FLASH: Self = Self(0x15);
}
impl CPUIRQSEL26_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL26_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL26_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x15 => f.write_str("FLASH"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL26_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x15 => defmt::write!(f, "FLASH"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL26_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL26_EV {
        CPUIRQSEL26_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL26_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL26_EV) -> u8 {
        CPUIRQSEL26_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL27_EV(u8);
impl CPUIRQSEL27_EV {
    #[doc = "Software event 0, triggered by SWEV.SWEV0."]
    pub const SWEV0: Self = Self(0x64);
}
impl CPUIRQSEL27_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL27_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL27_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x64 => f.write_str("SWEV0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL27_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x64 => defmt::write!(f, "SWEV0"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL27_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL27_EV {
        CPUIRQSEL27_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL27_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL27_EV) -> u8 {
        CPUIRQSEL27_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL28_EV(u8);
impl CPUIRQSEL28_EV {
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS."]
    pub const AUX_COMB: Self = Self(0x0b);
}
impl CPUIRQSEL28_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL28_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL28_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0b => f.write_str("AUX_COMB"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL28_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0b => defmt::write!(f, "AUX_COMB"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL28_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL28_EV {
        CPUIRQSEL28_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL28_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL28_EV) -> u8 {
        CPUIRQSEL28_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL29_EV(u8);
impl CPUIRQSEL29_EV {
    #[doc = "AON programmable event 0. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG0_EV."]
    pub const AON_PROG0: Self = Self(0x01);
}
impl CPUIRQSEL29_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL29_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL29_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("AON_PROG0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL29_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "AON_PROG0"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL29_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL29_EV {
        CPUIRQSEL29_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL29_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL29_EV) -> u8 {
        CPUIRQSEL29_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL2_EV(u8);
impl CPUIRQSEL2_EV {
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event."]
    pub const RFC_CPE_1: Self = Self(0x1e);
}
impl CPUIRQSEL2_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL2_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL2_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x1e => f.write_str("RFC_CPE_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL2_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x1e => defmt::write!(f, "RFC_CPE_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL2_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL2_EV {
        CPUIRQSEL2_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL2_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL2_EV) -> u8 {
        CPUIRQSEL2_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL30_EV(u8);
impl CPUIRQSEL30_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
    #[doc = "AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV."]
    pub const AON_PROG1: Self = Self(0x02);
    #[doc = "AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV."]
    pub const AON_PROG2: Self = Self(0x03);
    #[doc = "Interrupt event from I2S."]
    pub const I2S_IRQ: Self = Self(0x08);
    #[doc = "AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0."]
    pub const AON_AUX_SWEV0: Self = Self(0x0a);
    #[doc = "DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ."]
    pub const DMA_CH0_DONE: Self = Self(0x14);
    #[doc = "DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ."]
    pub const DMA_CH18_DONE: Self = Self(0x16);
    #[doc = "CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE."]
    pub const CRYPTO_DMA_DONE_IRQ: Self = Self(0x5e);
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV."]
    pub const AUX_AON_WU_EV: Self = Self(0x69);
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB."]
    pub const AUX_COMPB: Self = Self(0x6b);
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE."]
    pub const AUX_TDC_DONE: Self = Self(0x6c);
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV."]
    pub const AUX_TIMER0_EV: Self = Self(0x6d);
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV."]
    pub const AUX_TIMER1_EV: Self = Self(0x6e);
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE."]
    pub const AUX_SMPH_AUTOTAKE_DONE: Self = Self(0x6f);
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE."]
    pub const AUX_ADC_DONE: Self = Self(0x70);
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    pub const AUX_ADC_FIFO_ALMOST_FULL: Self = Self(0x71);
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0."]
    pub const AUX_OBSMUX0: Self = Self(0x72);
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN."]
    pub const AON_RTC_UPD: Self = Self(0x77);
    #[doc = "Always asserted."]
    pub const ALWAYS_ACTIVE: Self = Self(0x79);
}
impl CPUIRQSEL30_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL30_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL30_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            0x02 => f.write_str("AON_PROG1"),
            0x03 => f.write_str("AON_PROG2"),
            0x08 => f.write_str("I2S_IRQ"),
            0x0a => f.write_str("AON_AUX_SWEV0"),
            0x14 => f.write_str("DMA_CH0_DONE"),
            0x16 => f.write_str("DMA_CH18_DONE"),
            0x5e => f.write_str("CRYPTO_DMA_DONE_IRQ"),
            0x69 => f.write_str("AUX_AON_WU_EV"),
            0x6b => f.write_str("AUX_COMPB"),
            0x6c => f.write_str("AUX_TDC_DONE"),
            0x6d => f.write_str("AUX_TIMER0_EV"),
            0x6e => f.write_str("AUX_TIMER1_EV"),
            0x6f => f.write_str("AUX_SMPH_AUTOTAKE_DONE"),
            0x70 => f.write_str("AUX_ADC_DONE"),
            0x71 => f.write_str("AUX_ADC_FIFO_ALMOST_FULL"),
            0x72 => f.write_str("AUX_OBSMUX0"),
            0x77 => f.write_str("AON_RTC_UPD"),
            0x79 => f.write_str("ALWAYS_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL30_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            0x02 => defmt::write!(f, "AON_PROG1"),
            0x03 => defmt::write!(f, "AON_PROG2"),
            0x08 => defmt::write!(f, "I2S_IRQ"),
            0x0a => defmt::write!(f, "AON_AUX_SWEV0"),
            0x14 => defmt::write!(f, "DMA_CH0_DONE"),
            0x16 => defmt::write!(f, "DMA_CH18_DONE"),
            0x5e => defmt::write!(f, "CRYPTO_DMA_DONE_IRQ"),
            0x69 => defmt::write!(f, "AUX_AON_WU_EV"),
            0x6b => defmt::write!(f, "AUX_COMPB"),
            0x6c => defmt::write!(f, "AUX_TDC_DONE"),
            0x6d => defmt::write!(f, "AUX_TIMER0_EV"),
            0x6e => defmt::write!(f, "AUX_TIMER1_EV"),
            0x6f => defmt::write!(f, "AUX_SMPH_AUTOTAKE_DONE"),
            0x70 => defmt::write!(f, "AUX_ADC_DONE"),
            0x71 => defmt::write!(f, "AUX_ADC_FIFO_ALMOST_FULL"),
            0x72 => defmt::write!(f, "AUX_OBSMUX0"),
            0x77 => defmt::write!(f, "AON_RTC_UPD"),
            0x79 => defmt::write!(f, "ALWAYS_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL30_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL30_EV {
        CPUIRQSEL30_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL30_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL30_EV) -> u8 {
        CPUIRQSEL30_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL31_EV(u8);
impl CPUIRQSEL31_EV {
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA."]
    pub const AUX_COMPA: Self = Self(0x6a);
}
impl CPUIRQSEL31_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL31_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL31_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x6a => f.write_str("AUX_COMPA"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL31_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x6a => defmt::write!(f, "AUX_COMPA"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL31_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL31_EV {
        CPUIRQSEL31_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL31_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL31_EV) -> u8 {
        CPUIRQSEL31_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL32_EV(u8);
impl CPUIRQSEL32_EV {
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS."]
    pub const AUX_ADC_IRQ: Self = Self(0x73);
}
impl CPUIRQSEL32_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL32_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL32_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x73 => f.write_str("AUX_ADC_IRQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL32_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x73 => defmt::write!(f, "AUX_ADC_IRQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL32_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL32_EV {
        CPUIRQSEL32_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL32_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL32_EV) -> u8 {
        CPUIRQSEL32_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL33_EV(u8);
impl CPUIRQSEL33_EV {
    #[doc = "TRNG Interrupt event, controlled by TRNG:IRQEN.EN."]
    pub const TRNG_IRQ: Self = Self(0x68);
}
impl CPUIRQSEL33_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL33_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL33_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x68 => f.write_str("TRNG_IRQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL33_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x68 => defmt::write!(f, "TRNG_IRQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL33_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL33_EV {
        CPUIRQSEL33_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL33_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL33_EV) -> u8 {
        CPUIRQSEL33_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL4_EV(u8);
impl CPUIRQSEL4_EV {
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting."]
    pub const AON_RTC_COMB: Self = Self(0x07);
}
impl CPUIRQSEL4_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL4_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL4_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x07 => f.write_str("AON_RTC_COMB"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL4_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x07 => defmt::write!(f, "AON_RTC_COMB"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL4_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL4_EV {
        CPUIRQSEL4_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL4_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL4_EV) -> u8 {
        CPUIRQSEL4_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL5_EV(u8);
impl CPUIRQSEL5_EV {
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS."]
    pub const UART0_COMB: Self = Self(0x24);
}
impl CPUIRQSEL5_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL5_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL5_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x24 => f.write_str("UART0_COMB"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL5_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x24 => defmt::write!(f, "UART0_COMB"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL5_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL5_EV {
        CPUIRQSEL5_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL5_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL5_EV) -> u8 {
        CPUIRQSEL5_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL6_EV(u8);
impl CPUIRQSEL6_EV {
    #[doc = "AUX software event 0, triggered by AUX_EVCTL:SWEVSET.SWEV0, also available as AUX_EVENT0 AON wake up event. MCU domain wakeup control AON_EVENT:MCUWUSEL AUX domain wakeup control AON_EVENT:AUXWUSEL."]
    pub const AUX_SWEV0: Self = Self(0x1c);
}
impl CPUIRQSEL6_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL6_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL6_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x1c => f.write_str("AUX_SWEV0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL6_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x1c => defmt::write!(f, "AUX_SWEV0"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL6_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL6_EV {
        CPUIRQSEL6_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL6_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL6_EV) -> u8 {
        CPUIRQSEL6_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL7_EV(u8);
impl CPUIRQSEL7_EV {
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS."]
    pub const SSI0_COMB: Self = Self(0x22);
}
impl CPUIRQSEL7_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL7_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL7_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x22 => f.write_str("SSI0_COMB"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL7_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x22 => defmt::write!(f, "SSI0_COMB"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL7_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL7_EV {
        CPUIRQSEL7_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL7_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL7_EV) -> u8 {
        CPUIRQSEL7_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL8_EV(u8);
impl CPUIRQSEL8_EV {
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS."]
    pub const SSI1_COMB: Self = Self(0x23);
}
impl CPUIRQSEL8_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL8_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL8_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x23 => f.write_str("SSI1_COMB"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL8_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x23 => defmt::write!(f, "SSI1_COMB"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL8_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL8_EV {
        CPUIRQSEL8_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL8_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL8_EV) -> u8 {
        CPUIRQSEL8_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPUIRQSEL9_EV(u8);
impl CPUIRQSEL9_EV {
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event."]
    pub const RFC_CPE_0: Self = Self(0x1b);
}
impl CPUIRQSEL9_EV {
    pub const fn from_bits(val: u8) -> CPUIRQSEL9_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CPUIRQSEL9_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x1b => f.write_str("RFC_CPE_0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL9_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x1b => defmt::write!(f, "RFC_CPE_0"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CPUIRQSEL9_EV {
    #[inline(always)]
    fn from(val: u8) -> CPUIRQSEL9_EV {
        CPUIRQSEL9_EV::from_bits(val)
    }
}
impl From<CPUIRQSEL9_EV> for u8 {
    #[inline(always)]
    fn from(val: CPUIRQSEL9_EV) -> u8 {
        CPUIRQSEL9_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FRZSEL0_EV(u8);
impl FRZSEL0_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
    #[doc = "CPU halted."]
    pub const CPU_HALTED: Self = Self(0x78);
    #[doc = "Always asserted."]
    pub const ALWAYS_ACTIVE: Self = Self(0x79);
}
impl FRZSEL0_EV {
    pub const fn from_bits(val: u8) -> FRZSEL0_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for FRZSEL0_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            0x78 => f.write_str("CPU_HALTED"),
            0x79 => f.write_str("ALWAYS_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FRZSEL0_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            0x78 => defmt::write!(f, "CPU_HALTED"),
            0x79 => defmt::write!(f, "ALWAYS_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for FRZSEL0_EV {
    #[inline(always)]
    fn from(val: u8) -> FRZSEL0_EV {
        FRZSEL0_EV::from_bits(val)
    }
}
impl From<FRZSEL0_EV> for u8 {
    #[inline(always)]
    fn from(val: FRZSEL0_EV) -> u8 {
        FRZSEL0_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPT0ACAPTSEL_EV {
    #[doc = "Always inactive."]
    NONE = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings."]
    AON_GPIO_EDGE = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting."]
    AON_RTC_COMB = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Interrupt event from I2C."]
    I2C_IRQ = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS."]
    AUX_COMB = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT."]
    FLASH = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG."]
    RFC_CMD_ACK = 0x19,
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG."]
    RFC_HW_COMB = 0x1a,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event."]
    RFC_CPE_0 = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event."]
    RFC_CPE_1 = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS."]
    SSI0_COMB = 0x22,
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS."]
    SSI1_COMB = 0x23,
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS."]
    UART0_COMB = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT."]
    GPT0A_CMP = 0x3d,
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT."]
    GPT0B_CMP = 0x3e,
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT."]
    GPT1A_CMP = 0x3f,
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT."]
    GPT1B_CMP = 0x40,
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT."]
    GPT2A_CMP = 0x41,
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT."]
    GPT2B_CMP = 0x42,
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT."]
    GPT3A_CMP = 0x43,
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT."]
    GPT3B_CMP = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT0 wil be routed here."]
    PORT_EVENT0 = 0x55,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT1 wil be routed here."]
    PORT_EVENT1 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV."]
    AUX_AON_WU_EV = 0x69,
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA."]
    AUX_COMPA = 0x6a,
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB."]
    AUX_COMPB = 0x6b,
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE."]
    AUX_TDC_DONE = 0x6c,
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV."]
    AUX_TIMER0_EV = 0x6d,
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV."]
    AUX_TIMER1_EV = 0x6e,
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE."]
    AUX_SMPH_AUTOTAKE_DONE = 0x6f,
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE."]
    AUX_ADC_DONE = 0x70,
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    AUX_ADC_FIFO_ALMOST_FULL = 0x71,
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0."]
    AUX_OBSMUX0 = 0x72,
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS."]
    AUX_ADC_IRQ = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN."]
    AON_RTC_UPD = 0x77,
    _RESERVED_78 = 0x78,
    #[doc = "Always asserted."]
    ALWAYS_ACTIVE = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl GPT0ACAPTSEL_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPT0ACAPTSEL_EV {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPT0ACAPTSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> GPT0ACAPTSEL_EV {
        GPT0ACAPTSEL_EV::from_bits(val)
    }
}
impl From<GPT0ACAPTSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: GPT0ACAPTSEL_EV) -> u8 {
        GPT0ACAPTSEL_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPT0BCAPTSEL_EV {
    #[doc = "Always inactive."]
    NONE = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings."]
    AON_GPIO_EDGE = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting."]
    AON_RTC_COMB = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Interrupt event from I2C."]
    I2C_IRQ = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS."]
    AUX_COMB = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT."]
    FLASH = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG."]
    RFC_CMD_ACK = 0x19,
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG."]
    RFC_HW_COMB = 0x1a,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event."]
    RFC_CPE_0 = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event."]
    RFC_CPE_1 = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS."]
    SSI0_COMB = 0x22,
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS."]
    SSI1_COMB = 0x23,
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS."]
    UART0_COMB = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT."]
    GPT0A_CMP = 0x3d,
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT."]
    GPT0B_CMP = 0x3e,
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT."]
    GPT1A_CMP = 0x3f,
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT."]
    GPT1B_CMP = 0x40,
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT."]
    GPT2A_CMP = 0x41,
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT."]
    GPT2B_CMP = 0x42,
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT."]
    GPT3A_CMP = 0x43,
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT."]
    GPT3B_CMP = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT0 wil be routed here."]
    PORT_EVENT0 = 0x55,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT1 wil be routed here."]
    PORT_EVENT1 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV."]
    AUX_AON_WU_EV = 0x69,
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA."]
    AUX_COMPA = 0x6a,
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB."]
    AUX_COMPB = 0x6b,
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE."]
    AUX_TDC_DONE = 0x6c,
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV."]
    AUX_TIMER0_EV = 0x6d,
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV."]
    AUX_TIMER1_EV = 0x6e,
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE."]
    AUX_SMPH_AUTOTAKE_DONE = 0x6f,
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE."]
    AUX_ADC_DONE = 0x70,
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    AUX_ADC_FIFO_ALMOST_FULL = 0x71,
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0."]
    AUX_OBSMUX0 = 0x72,
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS."]
    AUX_ADC_IRQ = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN."]
    AON_RTC_UPD = 0x77,
    _RESERVED_78 = 0x78,
    #[doc = "Always asserted."]
    ALWAYS_ACTIVE = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl GPT0BCAPTSEL_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPT0BCAPTSEL_EV {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPT0BCAPTSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> GPT0BCAPTSEL_EV {
        GPT0BCAPTSEL_EV::from_bits(val)
    }
}
impl From<GPT0BCAPTSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: GPT0BCAPTSEL_EV) -> u8 {
        GPT0BCAPTSEL_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPT1ACAPTSEL_EV {
    #[doc = "Always inactive."]
    NONE = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings."]
    AON_GPIO_EDGE = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting."]
    AON_RTC_COMB = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Interrupt event from I2C."]
    I2C_IRQ = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS."]
    AUX_COMB = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT."]
    FLASH = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG."]
    RFC_CMD_ACK = 0x19,
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG."]
    RFC_HW_COMB = 0x1a,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event."]
    RFC_CPE_0 = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event."]
    RFC_CPE_1 = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS."]
    SSI0_COMB = 0x22,
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS."]
    SSI1_COMB = 0x23,
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS."]
    UART0_COMB = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT."]
    GPT0A_CMP = 0x3d,
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT."]
    GPT0B_CMP = 0x3e,
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT."]
    GPT1A_CMP = 0x3f,
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT."]
    GPT1B_CMP = 0x40,
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT."]
    GPT2A_CMP = 0x41,
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT."]
    GPT2B_CMP = 0x42,
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT."]
    GPT3A_CMP = 0x43,
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT."]
    GPT3B_CMP = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT2 wil be routed here."]
    PORT_EVENT2 = 0x57,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT3 wil be routed here."]
    PORT_EVENT3 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV."]
    AUX_AON_WU_EV = 0x69,
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA."]
    AUX_COMPA = 0x6a,
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB."]
    AUX_COMPB = 0x6b,
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE."]
    AUX_TDC_DONE = 0x6c,
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV."]
    AUX_TIMER0_EV = 0x6d,
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV."]
    AUX_TIMER1_EV = 0x6e,
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE."]
    AUX_SMPH_AUTOTAKE_DONE = 0x6f,
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE."]
    AUX_ADC_DONE = 0x70,
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    AUX_ADC_FIFO_ALMOST_FULL = 0x71,
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0."]
    AUX_OBSMUX0 = 0x72,
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS."]
    AUX_ADC_IRQ = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN."]
    AON_RTC_UPD = 0x77,
    _RESERVED_78 = 0x78,
    #[doc = "Always asserted."]
    ALWAYS_ACTIVE = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl GPT1ACAPTSEL_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPT1ACAPTSEL_EV {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPT1ACAPTSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> GPT1ACAPTSEL_EV {
        GPT1ACAPTSEL_EV::from_bits(val)
    }
}
impl From<GPT1ACAPTSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: GPT1ACAPTSEL_EV) -> u8 {
        GPT1ACAPTSEL_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPT1BCAPTSEL_EV {
    #[doc = "Always inactive."]
    NONE = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings."]
    AON_GPIO_EDGE = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting."]
    AON_RTC_COMB = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Interrupt event from I2C."]
    I2C_IRQ = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS."]
    AUX_COMB = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT."]
    FLASH = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG."]
    RFC_CMD_ACK = 0x19,
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG."]
    RFC_HW_COMB = 0x1a,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event."]
    RFC_CPE_0 = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event."]
    RFC_CPE_1 = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS."]
    SSI0_COMB = 0x22,
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS."]
    SSI1_COMB = 0x23,
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS."]
    UART0_COMB = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT."]
    GPT0A_CMP = 0x3d,
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT."]
    GPT0B_CMP = 0x3e,
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT."]
    GPT1A_CMP = 0x3f,
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT."]
    GPT1B_CMP = 0x40,
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT."]
    GPT2A_CMP = 0x41,
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT."]
    GPT2B_CMP = 0x42,
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT."]
    GPT3A_CMP = 0x43,
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT."]
    GPT3B_CMP = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT2 wil be routed here."]
    PORT_EVENT2 = 0x57,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT3 wil be routed here."]
    PORT_EVENT3 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV."]
    AUX_AON_WU_EV = 0x69,
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA."]
    AUX_COMPA = 0x6a,
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB."]
    AUX_COMPB = 0x6b,
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE."]
    AUX_TDC_DONE = 0x6c,
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV."]
    AUX_TIMER0_EV = 0x6d,
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV."]
    AUX_TIMER1_EV = 0x6e,
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE."]
    AUX_SMPH_AUTOTAKE_DONE = 0x6f,
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE."]
    AUX_ADC_DONE = 0x70,
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    AUX_ADC_FIFO_ALMOST_FULL = 0x71,
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0."]
    AUX_OBSMUX0 = 0x72,
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS."]
    AUX_ADC_IRQ = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN."]
    AON_RTC_UPD = 0x77,
    _RESERVED_78 = 0x78,
    #[doc = "Always asserted."]
    ALWAYS_ACTIVE = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl GPT1BCAPTSEL_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPT1BCAPTSEL_EV {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPT1BCAPTSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> GPT1BCAPTSEL_EV {
        GPT1BCAPTSEL_EV::from_bits(val)
    }
}
impl From<GPT1BCAPTSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: GPT1BCAPTSEL_EV) -> u8 {
        GPT1BCAPTSEL_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPT2ACAPTSEL_EV {
    #[doc = "Always inactive."]
    NONE = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings."]
    AON_GPIO_EDGE = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting."]
    AON_RTC_COMB = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Interrupt event from I2C."]
    I2C_IRQ = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS."]
    AUX_COMB = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT."]
    FLASH = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG."]
    RFC_CMD_ACK = 0x19,
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG."]
    RFC_HW_COMB = 0x1a,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event."]
    RFC_CPE_0 = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event."]
    RFC_CPE_1 = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS."]
    SSI0_COMB = 0x22,
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS."]
    SSI1_COMB = 0x23,
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS."]
    UART0_COMB = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT."]
    GPT0A_CMP = 0x3d,
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT."]
    GPT0B_CMP = 0x3e,
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT."]
    GPT1A_CMP = 0x3f,
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT."]
    GPT1B_CMP = 0x40,
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT."]
    GPT2A_CMP = 0x41,
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT."]
    GPT2B_CMP = 0x42,
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT."]
    GPT3A_CMP = 0x43,
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT."]
    GPT3B_CMP = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    PORT_EVENT4 = 0x59,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    PORT_EVENT5 = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV."]
    AUX_AON_WU_EV = 0x69,
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA."]
    AUX_COMPA = 0x6a,
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB."]
    AUX_COMPB = 0x6b,
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE."]
    AUX_TDC_DONE = 0x6c,
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV."]
    AUX_TIMER0_EV = 0x6d,
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV."]
    AUX_TIMER1_EV = 0x6e,
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE."]
    AUX_SMPH_AUTOTAKE_DONE = 0x6f,
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE."]
    AUX_ADC_DONE = 0x70,
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    AUX_ADC_FIFO_ALMOST_FULL = 0x71,
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0."]
    AUX_OBSMUX0 = 0x72,
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS."]
    AUX_ADC_IRQ = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN."]
    AON_RTC_UPD = 0x77,
    _RESERVED_78 = 0x78,
    #[doc = "Always asserted."]
    ALWAYS_ACTIVE = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl GPT2ACAPTSEL_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPT2ACAPTSEL_EV {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPT2ACAPTSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> GPT2ACAPTSEL_EV {
        GPT2ACAPTSEL_EV::from_bits(val)
    }
}
impl From<GPT2ACAPTSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: GPT2ACAPTSEL_EV) -> u8 {
        GPT2ACAPTSEL_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPT2BCAPTSEL_EV {
    #[doc = "Always inactive."]
    NONE = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings."]
    AON_GPIO_EDGE = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting."]
    AON_RTC_COMB = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Interrupt event from I2C."]
    I2C_IRQ = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS."]
    AUX_COMB = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT."]
    FLASH = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG."]
    RFC_CMD_ACK = 0x19,
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG."]
    RFC_HW_COMB = 0x1a,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event."]
    RFC_CPE_0 = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event."]
    RFC_CPE_1 = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS."]
    SSI0_COMB = 0x22,
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS."]
    SSI1_COMB = 0x23,
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS."]
    UART0_COMB = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT."]
    GPT0A_CMP = 0x3d,
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT."]
    GPT0B_CMP = 0x3e,
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT."]
    GPT1A_CMP = 0x3f,
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT."]
    GPT1B_CMP = 0x40,
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT."]
    GPT2A_CMP = 0x41,
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT."]
    GPT2B_CMP = 0x42,
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT."]
    GPT3A_CMP = 0x43,
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT."]
    GPT3B_CMP = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    PORT_EVENT4 = 0x59,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    PORT_EVENT5 = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV."]
    AUX_AON_WU_EV = 0x69,
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA."]
    AUX_COMPA = 0x6a,
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB."]
    AUX_COMPB = 0x6b,
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE."]
    AUX_TDC_DONE = 0x6c,
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV."]
    AUX_TIMER0_EV = 0x6d,
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV."]
    AUX_TIMER1_EV = 0x6e,
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE."]
    AUX_SMPH_AUTOTAKE_DONE = 0x6f,
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE."]
    AUX_ADC_DONE = 0x70,
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    AUX_ADC_FIFO_ALMOST_FULL = 0x71,
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0."]
    AUX_OBSMUX0 = 0x72,
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS."]
    AUX_ADC_IRQ = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN."]
    AON_RTC_UPD = 0x77,
    _RESERVED_78 = 0x78,
    #[doc = "Always asserted."]
    ALWAYS_ACTIVE = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl GPT2BCAPTSEL_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPT2BCAPTSEL_EV {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPT2BCAPTSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> GPT2BCAPTSEL_EV {
        GPT2BCAPTSEL_EV::from_bits(val)
    }
}
impl From<GPT2BCAPTSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: GPT2BCAPTSEL_EV) -> u8 {
        GPT2BCAPTSEL_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPT3ACAPTSEL_EV {
    #[doc = "Always inactive."]
    NONE = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings."]
    AON_GPIO_EDGE = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting."]
    AON_RTC_COMB = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS."]
    AUX_COMB = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT."]
    FLASH = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG."]
    RFC_CMD_ACK = 0x19,
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG."]
    RFC_HW_COMB = 0x1a,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event."]
    RFC_CPE_0 = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event."]
    RFC_CPE_1 = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS."]
    SSI0_COMB = 0x22,
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS."]
    SSI1_COMB = 0x23,
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS."]
    UART0_COMB = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT."]
    GPT0A_CMP = 0x3d,
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT."]
    GPT0B_CMP = 0x3e,
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT."]
    GPT1A_CMP = 0x3f,
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT."]
    GPT1B_CMP = 0x40,
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT."]
    GPT2A_CMP = 0x41,
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT."]
    GPT2B_CMP = 0x42,
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT."]
    GPT3A_CMP = 0x43,
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT."]
    GPT3B_CMP = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT6 wil be routed here."]
    PORT_EVENT6 = 0x5b,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT7 wil be routed here."]
    PORT_EVENT7 = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV."]
    AUX_AON_WU_EV = 0x69,
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA."]
    AUX_COMPA = 0x6a,
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB."]
    AUX_COMPB = 0x6b,
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE."]
    AUX_TDC_DONE = 0x6c,
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV."]
    AUX_TIMER0_EV = 0x6d,
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV."]
    AUX_TIMER1_EV = 0x6e,
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE."]
    AUX_SMPH_AUTOTAKE_DONE = 0x6f,
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE."]
    AUX_ADC_DONE = 0x70,
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    AUX_ADC_FIFO_ALMOST_FULL = 0x71,
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0."]
    AUX_OBSMUX0 = 0x72,
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS."]
    AUX_ADC_IRQ = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN."]
    AON_RTC_UPD = 0x77,
    _RESERVED_78 = 0x78,
    #[doc = "Always asserted."]
    ALWAYS_ACTIVE = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl GPT3ACAPTSEL_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPT3ACAPTSEL_EV {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPT3ACAPTSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> GPT3ACAPTSEL_EV {
        GPT3ACAPTSEL_EV::from_bits(val)
    }
}
impl From<GPT3ACAPTSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: GPT3ACAPTSEL_EV) -> u8 {
        GPT3ACAPTSEL_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPT3BCAPTSEL_EV {
    #[doc = "Always inactive."]
    NONE = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings."]
    AON_GPIO_EDGE = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting."]
    AON_RTC_COMB = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS."]
    AUX_COMB = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT."]
    FLASH = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG."]
    RFC_CMD_ACK = 0x19,
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG."]
    RFC_HW_COMB = 0x1a,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event."]
    RFC_CPE_0 = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event."]
    RFC_CPE_1 = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS."]
    SSI0_COMB = 0x22,
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS."]
    SSI1_COMB = 0x23,
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS."]
    UART0_COMB = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT."]
    GPT0A_CMP = 0x3d,
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT."]
    GPT0B_CMP = 0x3e,
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT."]
    GPT1A_CMP = 0x3f,
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT."]
    GPT1B_CMP = 0x40,
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT."]
    GPT2A_CMP = 0x41,
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT."]
    GPT2B_CMP = 0x42,
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT."]
    GPT3A_CMP = 0x43,
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT."]
    GPT3B_CMP = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT6 wil be routed here."]
    PORT_EVENT6 = 0x5b,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT7 wil be routed here."]
    PORT_EVENT7 = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV."]
    AUX_AON_WU_EV = 0x69,
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA."]
    AUX_COMPA = 0x6a,
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB."]
    AUX_COMPB = 0x6b,
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE."]
    AUX_TDC_DONE = 0x6c,
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV."]
    AUX_TIMER0_EV = 0x6d,
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV."]
    AUX_TIMER1_EV = 0x6e,
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE."]
    AUX_SMPH_AUTOTAKE_DONE = 0x6f,
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE."]
    AUX_ADC_DONE = 0x70,
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    AUX_ADC_FIFO_ALMOST_FULL = 0x71,
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0."]
    AUX_OBSMUX0 = 0x72,
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS."]
    AUX_ADC_IRQ = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN."]
    AON_RTC_UPD = 0x77,
    _RESERVED_78 = 0x78,
    #[doc = "Always asserted."]
    ALWAYS_ACTIVE = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl GPT3BCAPTSEL_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPT3BCAPTSEL_EV {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPT3BCAPTSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> GPT3BCAPTSEL_EV {
        GPT3BCAPTSEL_EV::from_bits(val)
    }
}
impl From<GPT3BCAPTSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: GPT3BCAPTSEL_EV) -> u8 {
        GPT3BCAPTSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct I2SSTMPSEL0_EV(u8);
impl I2SSTMPSEL0_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
    #[doc = "Always asserted."]
    pub const ALWAYS_ACTIVE: Self = Self(0x79);
}
impl I2SSTMPSEL0_EV {
    pub const fn from_bits(val: u8) -> I2SSTMPSEL0_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for I2SSTMPSEL0_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            0x79 => f.write_str("ALWAYS_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2SSTMPSEL0_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            0x79 => defmt::write!(f, "ALWAYS_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for I2SSTMPSEL0_EV {
    #[inline(always)]
    fn from(val: u8) -> I2SSTMPSEL0_EV {
        I2SSTMPSEL0_EV::from_bits(val)
    }
}
impl From<I2SSTMPSEL0_EV> for u8 {
    #[inline(always)]
    fn from(val: I2SSTMPSEL0_EV) -> u8 {
        I2SSTMPSEL0_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RFCSEL0_EV(u8);
impl RFCSEL0_EV {
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT."]
    pub const GPT0A_CMP: Self = Self(0x3d);
}
impl RFCSEL0_EV {
    pub const fn from_bits(val: u8) -> RFCSEL0_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for RFCSEL0_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x3d => f.write_str("GPT0A_CMP"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL0_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x3d => defmt::write!(f, "GPT0A_CMP"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for RFCSEL0_EV {
    #[inline(always)]
    fn from(val: u8) -> RFCSEL0_EV {
        RFCSEL0_EV::from_bits(val)
    }
}
impl From<RFCSEL0_EV> for u8 {
    #[inline(always)]
    fn from(val: RFCSEL0_EV) -> u8 {
        RFCSEL0_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RFCSEL1_EV(u8);
impl RFCSEL1_EV {
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT."]
    pub const GPT0B_CMP: Self = Self(0x3e);
}
impl RFCSEL1_EV {
    pub const fn from_bits(val: u8) -> RFCSEL1_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for RFCSEL1_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x3e => f.write_str("GPT0B_CMP"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL1_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x3e => defmt::write!(f, "GPT0B_CMP"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for RFCSEL1_EV {
    #[inline(always)]
    fn from(val: u8) -> RFCSEL1_EV {
        RFCSEL1_EV::from_bits(val)
    }
}
impl From<RFCSEL1_EV> for u8 {
    #[inline(always)]
    fn from(val: RFCSEL1_EV) -> u8 {
        RFCSEL1_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RFCSEL2_EV(u8);
impl RFCSEL2_EV {
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT."]
    pub const GPT1A_CMP: Self = Self(0x3f);
}
impl RFCSEL2_EV {
    pub const fn from_bits(val: u8) -> RFCSEL2_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for RFCSEL2_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x3f => f.write_str("GPT1A_CMP"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL2_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x3f => defmt::write!(f, "GPT1A_CMP"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for RFCSEL2_EV {
    #[inline(always)]
    fn from(val: u8) -> RFCSEL2_EV {
        RFCSEL2_EV::from_bits(val)
    }
}
impl From<RFCSEL2_EV> for u8 {
    #[inline(always)]
    fn from(val: RFCSEL2_EV) -> u8 {
        RFCSEL2_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RFCSEL3_EV(u8);
impl RFCSEL3_EV {
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT."]
    pub const GPT1B_CMP: Self = Self(0x40);
}
impl RFCSEL3_EV {
    pub const fn from_bits(val: u8) -> RFCSEL3_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for RFCSEL3_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x40 => f.write_str("GPT1B_CMP"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL3_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x40 => defmt::write!(f, "GPT1B_CMP"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for RFCSEL3_EV {
    #[inline(always)]
    fn from(val: u8) -> RFCSEL3_EV {
        RFCSEL3_EV::from_bits(val)
    }
}
impl From<RFCSEL3_EV> for u8 {
    #[inline(always)]
    fn from(val: RFCSEL3_EV) -> u8 {
        RFCSEL3_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RFCSEL4_EV(u8);
impl RFCSEL4_EV {
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT."]
    pub const GPT2A_CMP: Self = Self(0x41);
}
impl RFCSEL4_EV {
    pub const fn from_bits(val: u8) -> RFCSEL4_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for RFCSEL4_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x41 => f.write_str("GPT2A_CMP"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL4_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x41 => defmt::write!(f, "GPT2A_CMP"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for RFCSEL4_EV {
    #[inline(always)]
    fn from(val: u8) -> RFCSEL4_EV {
        RFCSEL4_EV::from_bits(val)
    }
}
impl From<RFCSEL4_EV> for u8 {
    #[inline(always)]
    fn from(val: RFCSEL4_EV) -> u8 {
        RFCSEL4_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RFCSEL5_EV(u8);
impl RFCSEL5_EV {
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT."]
    pub const GPT2B_CMP: Self = Self(0x42);
}
impl RFCSEL5_EV {
    pub const fn from_bits(val: u8) -> RFCSEL5_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for RFCSEL5_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x42 => f.write_str("GPT2B_CMP"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL5_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x42 => defmt::write!(f, "GPT2B_CMP"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for RFCSEL5_EV {
    #[inline(always)]
    fn from(val: u8) -> RFCSEL5_EV {
        RFCSEL5_EV::from_bits(val)
    }
}
impl From<RFCSEL5_EV> for u8 {
    #[inline(always)]
    fn from(val: RFCSEL5_EV) -> u8 {
        RFCSEL5_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RFCSEL6_EV(u8);
impl RFCSEL6_EV {
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT."]
    pub const GPT3A_CMP: Self = Self(0x43);
}
impl RFCSEL6_EV {
    pub const fn from_bits(val: u8) -> RFCSEL6_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for RFCSEL6_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x43 => f.write_str("GPT3A_CMP"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL6_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x43 => defmt::write!(f, "GPT3A_CMP"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for RFCSEL6_EV {
    #[inline(always)]
    fn from(val: u8) -> RFCSEL6_EV {
        RFCSEL6_EV::from_bits(val)
    }
}
impl From<RFCSEL6_EV> for u8 {
    #[inline(always)]
    fn from(val: RFCSEL6_EV) -> u8 {
        RFCSEL6_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RFCSEL7_EV(u8);
impl RFCSEL7_EV {
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT."]
    pub const GPT3B_CMP: Self = Self(0x44);
}
impl RFCSEL7_EV {
    pub const fn from_bits(val: u8) -> RFCSEL7_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for RFCSEL7_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x44 => f.write_str("GPT3B_CMP"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL7_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x44 => defmt::write!(f, "GPT3B_CMP"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for RFCSEL7_EV {
    #[inline(always)]
    fn from(val: u8) -> RFCSEL7_EV {
        RFCSEL7_EV::from_bits(val)
    }
}
impl From<RFCSEL7_EV> for u8 {
    #[inline(always)]
    fn from(val: RFCSEL7_EV) -> u8 {
        RFCSEL7_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RFCSEL8_EV(u8);
impl RFCSEL8_EV {
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN."]
    pub const AON_RTC_UPD: Self = Self(0x77);
}
impl RFCSEL8_EV {
    pub const fn from_bits(val: u8) -> RFCSEL8_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for RFCSEL8_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x77 => f.write_str("AON_RTC_UPD"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL8_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x77 => defmt::write!(f, "AON_RTC_UPD"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for RFCSEL8_EV {
    #[inline(always)]
    fn from(val: u8) -> RFCSEL8_EV {
        RFCSEL8_EV::from_bits(val)
    }
}
impl From<RFCSEL8_EV> for u8 {
    #[inline(always)]
    fn from(val: RFCSEL8_EV) -> u8 {
        RFCSEL8_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RFCSEL9_EV(u8);
impl RFCSEL9_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
    #[doc = "AON programmable event 0. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG0_EV."]
    pub const AON_PROG0: Self = Self(0x01);
    #[doc = "AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV."]
    pub const AON_PROG1: Self = Self(0x02);
    #[doc = "Interrupt event from I2S."]
    pub const I2S_IRQ: Self = Self(0x08);
    #[doc = "AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0."]
    pub const AON_AUX_SWEV0: Self = Self(0x0a);
    #[doc = "Watchdog interrupt event, controlled by WDT:CTL.INTEN."]
    pub const WDT_IRQ: Self = Self(0x18);
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS."]
    pub const SSI0_COMB: Self = Self(0x22);
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS."]
    pub const SSI1_COMB: Self = Self(0x23);
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS."]
    pub const UART0_COMB: Self = Self(0x24);
    #[doc = "Combined DMA done, corresponding flags are here UDMA0:REQDONE."]
    pub const DMA_DONE_COMB: Self = Self(0x27);
    #[doc = "CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL."]
    pub const CRYPTO_RESULT_AVAIL_IRQ: Self = Self(0x5d);
    #[doc = "Software event 0, triggered by SWEV.SWEV0."]
    pub const SWEV0: Self = Self(0x64);
    #[doc = "Software event 1, triggered by SWEV.SWEV1."]
    pub const SWEV1: Self = Self(0x65);
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV."]
    pub const AUX_AON_WU_EV: Self = Self(0x69);
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA."]
    pub const AUX_COMPA: Self = Self(0x6a);
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB."]
    pub const AUX_COMPB: Self = Self(0x6b);
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE."]
    pub const AUX_TDC_DONE: Self = Self(0x6c);
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV."]
    pub const AUX_TIMER0_EV: Self = Self(0x6d);
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV."]
    pub const AUX_TIMER1_EV: Self = Self(0x6e);
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE."]
    pub const AUX_SMPH_AUTOTAKE_DONE: Self = Self(0x6f);
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE."]
    pub const AUX_ADC_DONE: Self = Self(0x70);
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    pub const AUX_ADC_FIFO_ALMOST_FULL: Self = Self(0x71);
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0."]
    pub const AUX_OBSMUX0: Self = Self(0x72);
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS."]
    pub const AUX_ADC_IRQ: Self = Self(0x73);
    #[doc = "Always asserted."]
    pub const ALWAYS_ACTIVE: Self = Self(0x79);
}
impl RFCSEL9_EV {
    pub const fn from_bits(val: u8) -> RFCSEL9_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for RFCSEL9_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            0x01 => f.write_str("AON_PROG0"),
            0x02 => f.write_str("AON_PROG1"),
            0x08 => f.write_str("I2S_IRQ"),
            0x0a => f.write_str("AON_AUX_SWEV0"),
            0x18 => f.write_str("WDT_IRQ"),
            0x22 => f.write_str("SSI0_COMB"),
            0x23 => f.write_str("SSI1_COMB"),
            0x24 => f.write_str("UART0_COMB"),
            0x27 => f.write_str("DMA_DONE_COMB"),
            0x5d => f.write_str("CRYPTO_RESULT_AVAIL_IRQ"),
            0x64 => f.write_str("SWEV0"),
            0x65 => f.write_str("SWEV1"),
            0x69 => f.write_str("AUX_AON_WU_EV"),
            0x6a => f.write_str("AUX_COMPA"),
            0x6b => f.write_str("AUX_COMPB"),
            0x6c => f.write_str("AUX_TDC_DONE"),
            0x6d => f.write_str("AUX_TIMER0_EV"),
            0x6e => f.write_str("AUX_TIMER1_EV"),
            0x6f => f.write_str("AUX_SMPH_AUTOTAKE_DONE"),
            0x70 => f.write_str("AUX_ADC_DONE"),
            0x71 => f.write_str("AUX_ADC_FIFO_ALMOST_FULL"),
            0x72 => f.write_str("AUX_OBSMUX0"),
            0x73 => f.write_str("AUX_ADC_IRQ"),
            0x79 => f.write_str("ALWAYS_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL9_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            0x01 => defmt::write!(f, "AON_PROG0"),
            0x02 => defmt::write!(f, "AON_PROG1"),
            0x08 => defmt::write!(f, "I2S_IRQ"),
            0x0a => defmt::write!(f, "AON_AUX_SWEV0"),
            0x18 => defmt::write!(f, "WDT_IRQ"),
            0x22 => defmt::write!(f, "SSI0_COMB"),
            0x23 => defmt::write!(f, "SSI1_COMB"),
            0x24 => defmt::write!(f, "UART0_COMB"),
            0x27 => defmt::write!(f, "DMA_DONE_COMB"),
            0x5d => defmt::write!(f, "CRYPTO_RESULT_AVAIL_IRQ"),
            0x64 => defmt::write!(f, "SWEV0"),
            0x65 => defmt::write!(f, "SWEV1"),
            0x69 => defmt::write!(f, "AUX_AON_WU_EV"),
            0x6a => defmt::write!(f, "AUX_COMPA"),
            0x6b => defmt::write!(f, "AUX_COMPB"),
            0x6c => defmt::write!(f, "AUX_TDC_DONE"),
            0x6d => defmt::write!(f, "AUX_TIMER0_EV"),
            0x6e => defmt::write!(f, "AUX_TIMER1_EV"),
            0x6f => defmt::write!(f, "AUX_SMPH_AUTOTAKE_DONE"),
            0x70 => defmt::write!(f, "AUX_ADC_DONE"),
            0x71 => defmt::write!(f, "AUX_ADC_FIFO_ALMOST_FULL"),
            0x72 => defmt::write!(f, "AUX_OBSMUX0"),
            0x73 => defmt::write!(f, "AUX_ADC_IRQ"),
            0x79 => defmt::write!(f, "ALWAYS_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for RFCSEL9_EV {
    #[inline(always)]
    fn from(val: u8) -> RFCSEL9_EV {
        RFCSEL9_EV::from_bits(val)
    }
}
impl From<RFCSEL9_EV> for u8 {
    #[inline(always)]
    fn from(val: RFCSEL9_EV) -> u8 {
        RFCSEL9_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH0BSEL_EV(u32);
impl UDMACH0BSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH0BSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH0BSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH0BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH0BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH0BSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH0BSEL_EV {
        UDMACH0BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH0BSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH0BSEL_EV) -> u32 {
        UDMACH0BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH0SSEL_EV(u32);
impl UDMACH0SSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH0SSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH0SSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH0SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH0SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH0SSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH0SSEL_EV {
        UDMACH0SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH0SSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH0SSEL_EV) -> u32 {
        UDMACH0SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH10BSEL_EV(u8);
impl UDMACH10BSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV."]
    pub const GPT0A_DMABREQ: Self = Self(0x4d);
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV."]
    pub const GPT0B_DMABREQ: Self = Self(0x4e);
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV."]
    pub const GPT1A_DMABREQ: Self = Self(0x4f);
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV."]
    pub const GPT1B_DMABREQ: Self = Self(0x50);
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV."]
    pub const GPT2A_DMABREQ: Self = Self(0x51);
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV."]
    pub const GPT2B_DMABREQ: Self = Self(0x52);
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV."]
    pub const GPT3A_DMABREQ: Self = Self(0x53);
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV."]
    pub const GPT3B_DMABREQ: Self = Self(0x54);
    #[doc = "Always asserted."]
    pub const ALWAYS_ACTIVE: Self = Self(0x79);
}
impl UDMACH10BSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH10BSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH10BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            0x4d => f.write_str("GPT0A_DMABREQ"),
            0x4e => f.write_str("GPT0B_DMABREQ"),
            0x4f => f.write_str("GPT1A_DMABREQ"),
            0x50 => f.write_str("GPT1B_DMABREQ"),
            0x51 => f.write_str("GPT2A_DMABREQ"),
            0x52 => f.write_str("GPT2B_DMABREQ"),
            0x53 => f.write_str("GPT3A_DMABREQ"),
            0x54 => f.write_str("GPT3B_DMABREQ"),
            0x79 => f.write_str("ALWAYS_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH10BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            0x4d => defmt::write!(f, "GPT0A_DMABREQ"),
            0x4e => defmt::write!(f, "GPT0B_DMABREQ"),
            0x4f => defmt::write!(f, "GPT1A_DMABREQ"),
            0x50 => defmt::write!(f, "GPT1B_DMABREQ"),
            0x51 => defmt::write!(f, "GPT2A_DMABREQ"),
            0x52 => defmt::write!(f, "GPT2B_DMABREQ"),
            0x53 => defmt::write!(f, "GPT3A_DMABREQ"),
            0x54 => defmt::write!(f, "GPT3B_DMABREQ"),
            0x79 => defmt::write!(f, "ALWAYS_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH10BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH10BSEL_EV {
        UDMACH10BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH10BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH10BSEL_EV) -> u8 {
        UDMACH10BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH10SSEL_EV(u8);
impl UDMACH10SSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
    #[doc = "Not used tied to 0."]
    pub const TIE_LOW: Self = Self(0x46);
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV."]
    pub const GPT0A_DMABREQ: Self = Self(0x4d);
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV."]
    pub const GPT0B_DMABREQ: Self = Self(0x4e);
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV."]
    pub const GPT1A_DMABREQ: Self = Self(0x4f);
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV."]
    pub const GPT1B_DMABREQ: Self = Self(0x50);
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV."]
    pub const GPT2A_DMABREQ: Self = Self(0x51);
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV."]
    pub const GPT2B_DMABREQ: Self = Self(0x52);
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV."]
    pub const GPT3A_DMABREQ: Self = Self(0x53);
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV."]
    pub const GPT3B_DMABREQ: Self = Self(0x54);
    #[doc = "Always asserted."]
    pub const ALWAYS_ACTIVE: Self = Self(0x79);
}
impl UDMACH10SSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH10SSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH10SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            0x46 => f.write_str("TIE_LOW"),
            0x4d => f.write_str("GPT0A_DMABREQ"),
            0x4e => f.write_str("GPT0B_DMABREQ"),
            0x4f => f.write_str("GPT1A_DMABREQ"),
            0x50 => f.write_str("GPT1B_DMABREQ"),
            0x51 => f.write_str("GPT2A_DMABREQ"),
            0x52 => f.write_str("GPT2B_DMABREQ"),
            0x53 => f.write_str("GPT3A_DMABREQ"),
            0x54 => f.write_str("GPT3B_DMABREQ"),
            0x79 => f.write_str("ALWAYS_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH10SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            0x46 => defmt::write!(f, "TIE_LOW"),
            0x4d => defmt::write!(f, "GPT0A_DMABREQ"),
            0x4e => defmt::write!(f, "GPT0B_DMABREQ"),
            0x4f => defmt::write!(f, "GPT1A_DMABREQ"),
            0x50 => defmt::write!(f, "GPT1B_DMABREQ"),
            0x51 => defmt::write!(f, "GPT2A_DMABREQ"),
            0x52 => defmt::write!(f, "GPT2B_DMABREQ"),
            0x53 => defmt::write!(f, "GPT3A_DMABREQ"),
            0x54 => defmt::write!(f, "GPT3B_DMABREQ"),
            0x79 => defmt::write!(f, "ALWAYS_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH10SSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH10SSEL_EV {
        UDMACH10SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH10SSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH10SSEL_EV) -> u8 {
        UDMACH10SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH11BSEL_EV(u8);
impl UDMACH11BSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV."]
    pub const GPT0A_DMABREQ: Self = Self(0x4d);
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV."]
    pub const GPT0B_DMABREQ: Self = Self(0x4e);
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV."]
    pub const GPT1A_DMABREQ: Self = Self(0x4f);
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV."]
    pub const GPT1B_DMABREQ: Self = Self(0x50);
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV."]
    pub const GPT2A_DMABREQ: Self = Self(0x51);
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV."]
    pub const GPT2B_DMABREQ: Self = Self(0x52);
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV."]
    pub const GPT3A_DMABREQ: Self = Self(0x53);
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV."]
    pub const GPT3B_DMABREQ: Self = Self(0x54);
    #[doc = "Always asserted."]
    pub const ALWAYS_ACTIVE: Self = Self(0x79);
}
impl UDMACH11BSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH11BSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH11BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            0x4d => f.write_str("GPT0A_DMABREQ"),
            0x4e => f.write_str("GPT0B_DMABREQ"),
            0x4f => f.write_str("GPT1A_DMABREQ"),
            0x50 => f.write_str("GPT1B_DMABREQ"),
            0x51 => f.write_str("GPT2A_DMABREQ"),
            0x52 => f.write_str("GPT2B_DMABREQ"),
            0x53 => f.write_str("GPT3A_DMABREQ"),
            0x54 => f.write_str("GPT3B_DMABREQ"),
            0x79 => f.write_str("ALWAYS_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH11BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            0x4d => defmt::write!(f, "GPT0A_DMABREQ"),
            0x4e => defmt::write!(f, "GPT0B_DMABREQ"),
            0x4f => defmt::write!(f, "GPT1A_DMABREQ"),
            0x50 => defmt::write!(f, "GPT1B_DMABREQ"),
            0x51 => defmt::write!(f, "GPT2A_DMABREQ"),
            0x52 => defmt::write!(f, "GPT2B_DMABREQ"),
            0x53 => defmt::write!(f, "GPT3A_DMABREQ"),
            0x54 => defmt::write!(f, "GPT3B_DMABREQ"),
            0x79 => defmt::write!(f, "ALWAYS_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH11BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH11BSEL_EV {
        UDMACH11BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH11BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH11BSEL_EV) -> u8 {
        UDMACH11BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH11SSEL_EV(u8);
impl UDMACH11SSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
    #[doc = "Not used tied to 0."]
    pub const TIE_LOW: Self = Self(0x47);
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV."]
    pub const GPT0A_DMABREQ: Self = Self(0x4d);
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV."]
    pub const GPT0B_DMABREQ: Self = Self(0x4e);
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV."]
    pub const GPT1A_DMABREQ: Self = Self(0x4f);
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV."]
    pub const GPT1B_DMABREQ: Self = Self(0x50);
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV."]
    pub const GPT2A_DMABREQ: Self = Self(0x51);
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV."]
    pub const GPT2B_DMABREQ: Self = Self(0x52);
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV."]
    pub const GPT3A_DMABREQ: Self = Self(0x53);
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV."]
    pub const GPT3B_DMABREQ: Self = Self(0x54);
    #[doc = "Always asserted."]
    pub const ALWAYS_ACTIVE: Self = Self(0x79);
}
impl UDMACH11SSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH11SSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH11SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            0x47 => f.write_str("TIE_LOW"),
            0x4d => f.write_str("GPT0A_DMABREQ"),
            0x4e => f.write_str("GPT0B_DMABREQ"),
            0x4f => f.write_str("GPT1A_DMABREQ"),
            0x50 => f.write_str("GPT1B_DMABREQ"),
            0x51 => f.write_str("GPT2A_DMABREQ"),
            0x52 => f.write_str("GPT2B_DMABREQ"),
            0x53 => f.write_str("GPT3A_DMABREQ"),
            0x54 => f.write_str("GPT3B_DMABREQ"),
            0x79 => f.write_str("ALWAYS_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH11SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            0x47 => defmt::write!(f, "TIE_LOW"),
            0x4d => defmt::write!(f, "GPT0A_DMABREQ"),
            0x4e => defmt::write!(f, "GPT0B_DMABREQ"),
            0x4f => defmt::write!(f, "GPT1A_DMABREQ"),
            0x50 => defmt::write!(f, "GPT1B_DMABREQ"),
            0x51 => defmt::write!(f, "GPT2A_DMABREQ"),
            0x52 => defmt::write!(f, "GPT2B_DMABREQ"),
            0x53 => defmt::write!(f, "GPT3A_DMABREQ"),
            0x54 => defmt::write!(f, "GPT3B_DMABREQ"),
            0x79 => defmt::write!(f, "ALWAYS_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH11SSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH11SSEL_EV {
        UDMACH11SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH11SSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH11SSEL_EV) -> u8 {
        UDMACH11SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH12BSEL_EV(u8);
impl UDMACH12BSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV."]
    pub const GPT0A_DMABREQ: Self = Self(0x4d);
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV."]
    pub const GPT0B_DMABREQ: Self = Self(0x4e);
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV."]
    pub const GPT1A_DMABREQ: Self = Self(0x4f);
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV."]
    pub const GPT1B_DMABREQ: Self = Self(0x50);
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV."]
    pub const GPT2A_DMABREQ: Self = Self(0x51);
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV."]
    pub const GPT2B_DMABREQ: Self = Self(0x52);
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV."]
    pub const GPT3A_DMABREQ: Self = Self(0x53);
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV."]
    pub const GPT3B_DMABREQ: Self = Self(0x54);
    #[doc = "Always asserted."]
    pub const ALWAYS_ACTIVE: Self = Self(0x79);
}
impl UDMACH12BSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH12BSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH12BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            0x4d => f.write_str("GPT0A_DMABREQ"),
            0x4e => f.write_str("GPT0B_DMABREQ"),
            0x4f => f.write_str("GPT1A_DMABREQ"),
            0x50 => f.write_str("GPT1B_DMABREQ"),
            0x51 => f.write_str("GPT2A_DMABREQ"),
            0x52 => f.write_str("GPT2B_DMABREQ"),
            0x53 => f.write_str("GPT3A_DMABREQ"),
            0x54 => f.write_str("GPT3B_DMABREQ"),
            0x79 => f.write_str("ALWAYS_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH12BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            0x4d => defmt::write!(f, "GPT0A_DMABREQ"),
            0x4e => defmt::write!(f, "GPT0B_DMABREQ"),
            0x4f => defmt::write!(f, "GPT1A_DMABREQ"),
            0x50 => defmt::write!(f, "GPT1B_DMABREQ"),
            0x51 => defmt::write!(f, "GPT2A_DMABREQ"),
            0x52 => defmt::write!(f, "GPT2B_DMABREQ"),
            0x53 => defmt::write!(f, "GPT3A_DMABREQ"),
            0x54 => defmt::write!(f, "GPT3B_DMABREQ"),
            0x79 => defmt::write!(f, "ALWAYS_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH12BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH12BSEL_EV {
        UDMACH12BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH12BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH12BSEL_EV) -> u8 {
        UDMACH12BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH12SSEL_EV(u8);
impl UDMACH12SSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
    #[doc = "Not used tied to 0."]
    pub const TIE_LOW: Self = Self(0x48);
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV."]
    pub const GPT0A_DMABREQ: Self = Self(0x4d);
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV."]
    pub const GPT0B_DMABREQ: Self = Self(0x4e);
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV."]
    pub const GPT1A_DMABREQ: Self = Self(0x4f);
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV."]
    pub const GPT1B_DMABREQ: Self = Self(0x50);
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV."]
    pub const GPT2A_DMABREQ: Self = Self(0x51);
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV."]
    pub const GPT2B_DMABREQ: Self = Self(0x52);
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV."]
    pub const GPT3A_DMABREQ: Self = Self(0x53);
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV."]
    pub const GPT3B_DMABREQ: Self = Self(0x54);
    #[doc = "Always asserted."]
    pub const ALWAYS_ACTIVE: Self = Self(0x79);
}
impl UDMACH12SSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH12SSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH12SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            0x48 => f.write_str("TIE_LOW"),
            0x4d => f.write_str("GPT0A_DMABREQ"),
            0x4e => f.write_str("GPT0B_DMABREQ"),
            0x4f => f.write_str("GPT1A_DMABREQ"),
            0x50 => f.write_str("GPT1B_DMABREQ"),
            0x51 => f.write_str("GPT2A_DMABREQ"),
            0x52 => f.write_str("GPT2B_DMABREQ"),
            0x53 => f.write_str("GPT3A_DMABREQ"),
            0x54 => f.write_str("GPT3B_DMABREQ"),
            0x79 => f.write_str("ALWAYS_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH12SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            0x48 => defmt::write!(f, "TIE_LOW"),
            0x4d => defmt::write!(f, "GPT0A_DMABREQ"),
            0x4e => defmt::write!(f, "GPT0B_DMABREQ"),
            0x4f => defmt::write!(f, "GPT1A_DMABREQ"),
            0x50 => defmt::write!(f, "GPT1B_DMABREQ"),
            0x51 => defmt::write!(f, "GPT2A_DMABREQ"),
            0x52 => defmt::write!(f, "GPT2B_DMABREQ"),
            0x53 => defmt::write!(f, "GPT3A_DMABREQ"),
            0x54 => defmt::write!(f, "GPT3B_DMABREQ"),
            0x79 => defmt::write!(f, "ALWAYS_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH12SSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH12SSEL_EV {
        UDMACH12SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH12SSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH12SSEL_EV) -> u8 {
        UDMACH12SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH13BSEL_EV(u8);
impl UDMACH13BSEL_EV {
    #[doc = "AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV."]
    pub const AON_PROG2: Self = Self(0x03);
}
impl UDMACH13BSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH13BSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH13BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x03 => f.write_str("AON_PROG2"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH13BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x03 => defmt::write!(f, "AON_PROG2"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH13BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH13BSEL_EV {
        UDMACH13BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH13BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH13BSEL_EV) -> u8 {
        UDMACH13BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH13SSEL_EV(u32);
impl UDMACH13SSEL_EV {
    #[doc = "AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV."]
    pub const AON_PROG2: Self = Self(0x03);
}
impl UDMACH13SSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH13SSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH13SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x03 => f.write_str("AON_PROG2"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH13SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x03 => defmt::write!(f, "AON_PROG2"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH13SSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH13SSEL_EV {
        UDMACH13SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH13SSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH13SSEL_EV) -> u32 {
        UDMACH13SSEL_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UDMACH14BSEL_EV {
    #[doc = "Always inactive."]
    NONE = 0x0,
    #[doc = "AON programmable event 0. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG0_EV."]
    AON_PROG0 = 0x01,
    #[doc = "AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV."]
    AON_PROG1 = 0x02,
    #[doc = "AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV."]
    AON_PROG2 = 0x03,
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings."]
    AON_GPIO_EDGE = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting."]
    AON_RTC_COMB = 0x07,
    #[doc = "Interrupt event from I2S."]
    I2S_IRQ = 0x08,
    #[doc = "Interrupt event from I2C."]
    I2C_IRQ = 0x09,
    #[doc = "AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0."]
    AON_AUX_SWEV0 = 0x0a,
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS."]
    AUX_COMB = 0x0b,
    #[doc = "GPT2A interrupt event, controlled by GPT2:TAMR."]
    GPT2A = 0x0c,
    #[doc = "GPT2B interrupt event, controlled by GPT2:TBMR."]
    GPT2B = 0x0d,
    #[doc = "GPT3A interrupt event, controlled by GPT3:TAMR."]
    GPT3A = 0x0e,
    #[doc = "GPT3B interrupt event, controlled by GPT3:TBMR."]
    GPT3B = 0x0f,
    #[doc = "GPT0A interrupt event, controlled by GPT0:TAMR."]
    GPT0A = 0x10,
    #[doc = "GPT0B interrupt event, controlled by GPT0:TBMR."]
    GPT0B = 0x11,
    #[doc = "GPT1A interrupt event, controlled by GPT1:TAMR."]
    GPT1A = 0x12,
    #[doc = "GPT1B interrupt event, controlled by GPT1:TBMR."]
    GPT1B = 0x13,
    #[doc = "DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ."]
    DMA_CH0_DONE = 0x14,
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT."]
    FLASH = 0x15,
    #[doc = "DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ."]
    DMA_CH18_DONE = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "Watchdog interrupt event, controlled by WDT:CTL.INTEN."]
    WDT_IRQ = 0x18,
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG."]
    RFC_CMD_ACK = 0x19,
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG."]
    RFC_HW_COMB = 0x1a,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event."]
    RFC_CPE_0 = 0x1b,
    _RESERVED_1c = 0x1c,
    #[doc = "AUX software event 1, triggered by AUX_EVCTL:SWEVSET.SWEV1, also available as AUX_EVENT2 AON wake up event. MCU domain wakeup control AON_EVENT:MCUWUSEL AUX domain wakeup control AON_EVENT:AUXWUSEL."]
    AUX_SWEV1 = 0x1d,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event."]
    RFC_CPE_1 = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS."]
    SSI0_COMB = 0x22,
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS."]
    SSI1_COMB = 0x23,
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS."]
    UART0_COMB = 0x24,
    _RESERVED_25 = 0x25,
    #[doc = "DMA bus error, corresponds to UDMA0:ERROR.STATUS."]
    DMA_ERR = 0x26,
    #[doc = "Combined DMA done, corresponding flags are here UDMA0:REQDONE."]
    DMA_DONE_COMB = 0x27,
    #[doc = "SSI0 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE."]
    SSI0_RX_DMABREQ = 0x28,
    #[doc = "SSI0 RX DMA single request, controlled by SSI0:DMACR.RXDMAE."]
    SSI0_RX_DMASREQ = 0x29,
    #[doc = "SSI0 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE."]
    SSI0_TX_DMABREQ = 0x2a,
    #[doc = "SSI0 TX DMA single request, controlled by SSI0:DMACR.TXDMAE."]
    SSI0_TX_DMASREQ = 0x2b,
    #[doc = "SSI1 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE."]
    SSI1_RX_DMABREQ = 0x2c,
    #[doc = "SSI1 RX DMA single request, controlled by SSI0:DMACR.RXDMAE."]
    SSI1_RX_DMASREQ = 0x2d,
    #[doc = "SSI1 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE."]
    SSI1_TX_DMABREQ = 0x2e,
    #[doc = "SSI1 TX DMA single request, controlled by SSI0:DMACR.TXDMAE."]
    SSI1_TX_DMASREQ = 0x2f,
    #[doc = "UART0 RX DMA burst request, controlled by UART0:DMACTL.RXDMAE."]
    UART0_RX_DMABREQ = 0x30,
    #[doc = "UART0 RX DMA single request, controlled by UART0:DMACTL.RXDMAE."]
    UART0_RX_DMASREQ = 0x31,
    #[doc = "UART0 TX DMA burst request, controlled by UART0:DMACTL.TXDMAE."]
    UART0_TX_DMABREQ = 0x32,
    #[doc = "UART0 TX DMA single request, controlled by UART0:DMACTL.TXDMAE."]
    UART0_TX_DMASREQ = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT."]
    GPT0A_CMP = 0x3d,
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT."]
    GPT0B_CMP = 0x3e,
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT."]
    GPT1A_CMP = 0x3f,
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT."]
    GPT1B_CMP = 0x40,
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT."]
    GPT2A_CMP = 0x41,
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT."]
    GPT2B_CMP = 0x42,
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT."]
    GPT3A_CMP = 0x43,
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT."]
    GPT3B_CMP = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV."]
    GPT0A_DMABREQ = 0x4d,
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV."]
    GPT0B_DMABREQ = 0x4e,
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV."]
    GPT1A_DMABREQ = 0x4f,
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV."]
    GPT1B_DMABREQ = 0x50,
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV."]
    GPT2A_DMABREQ = 0x51,
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV."]
    GPT2B_DMABREQ = 0x52,
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV."]
    GPT3A_DMABREQ = 0x53,
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV."]
    GPT3B_DMABREQ = 0x54,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT0 wil be routed here."]
    PORT_EVENT0 = 0x55,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT1 wil be routed here."]
    PORT_EVENT1 = 0x56,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT2 wil be routed here."]
    PORT_EVENT2 = 0x57,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT3 wil be routed here."]
    PORT_EVENT3 = 0x58,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    PORT_EVENT4 = 0x59,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    PORT_EVENT5 = 0x5a,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT6 wil be routed here."]
    PORT_EVENT6 = 0x5b,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT7 wil be routed here."]
    PORT_EVENT7 = 0x5c,
    #[doc = "CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL."]
    CRYPTO_RESULT_AVAIL_IRQ = 0x5d,
    #[doc = "CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE."]
    CRYPTO_DMA_DONE_IRQ = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    #[doc = "Watchdog non maskable interrupt event, controlled by WDT:CTL.INTTYPE."]
    WDT_NMI = 0x63,
    #[doc = "Software event 0, triggered by SWEV.SWEV0."]
    SWEV0 = 0x64,
    #[doc = "Software event 1, triggered by SWEV.SWEV1."]
    SWEV1 = 0x65,
    #[doc = "Software event 2, triggered by SWEV.SWEV2."]
    SWEV2 = 0x66,
    #[doc = "Software event 3, triggered by SWEV.SWEV3."]
    SWEV3 = 0x67,
    #[doc = "TRNG Interrupt event, controlled by TRNG:IRQEN.EN."]
    TRNG_IRQ = 0x68,
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV."]
    AUX_AON_WU_EV = 0x69,
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA."]
    AUX_COMPA = 0x6a,
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB."]
    AUX_COMPB = 0x6b,
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE."]
    AUX_TDC_DONE = 0x6c,
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV."]
    AUX_TIMER0_EV = 0x6d,
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV."]
    AUX_TIMER1_EV = 0x6e,
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE."]
    AUX_SMPH_AUTOTAKE_DONE = 0x6f,
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE."]
    AUX_ADC_DONE = 0x70,
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    AUX_ADC_FIFO_ALMOST_FULL = 0x71,
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0."]
    AUX_OBSMUX0 = 0x72,
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS."]
    AUX_ADC_IRQ = 0x73,
    #[doc = "DMA sofware trigger from AUX, triggered by AUX_EVCTL:DMASWREQ.START."]
    AUX_SW_DMABREQ = 0x74,
    #[doc = "DMA single request event from AUX, configured by AUX_EVCTL:DMACTL."]
    AUX_DMASREQ = 0x75,
    #[doc = "DMA burst request event from AUX, configured by AUX_EVCTL:DMACTL."]
    AUX_DMABREQ = 0x76,
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN."]
    AON_RTC_UPD = 0x77,
    #[doc = "CPU halted."]
    CPU_HALTED = 0x78,
    #[doc = "Always asserted."]
    ALWAYS_ACTIVE = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl UDMACH14BSEL_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UDMACH14BSEL_EV {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UDMACH14BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH14BSEL_EV {
        UDMACH14BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH14BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH14BSEL_EV) -> u8 {
        UDMACH14BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH14SSEL_EV(u32);
impl UDMACH14SSEL_EV {
    #[doc = "AON programmable event 0. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG0_EV."]
    pub const AON_PROG0: Self = Self(0x01);
}
impl UDMACH14SSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH14SSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH14SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("AON_PROG0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH14SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "AON_PROG0"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH14SSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH14SSEL_EV {
        UDMACH14SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH14SSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH14SSEL_EV) -> u32 {
        UDMACH14SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH15BSEL_EV(u8);
impl UDMACH15BSEL_EV {
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting."]
    pub const AON_RTC_COMB: Self = Self(0x07);
}
impl UDMACH15BSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH15BSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH15BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x07 => f.write_str("AON_RTC_COMB"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH15BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x07 => defmt::write!(f, "AON_RTC_COMB"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH15BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH15BSEL_EV {
        UDMACH15BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH15BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH15BSEL_EV) -> u8 {
        UDMACH15BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH15SSEL_EV(u32);
impl UDMACH15SSEL_EV {
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting."]
    pub const AON_RTC_COMB: Self = Self(0x07);
}
impl UDMACH15SSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH15SSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH15SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x07 => f.write_str("AON_RTC_COMB"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH15SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x07 => defmt::write!(f, "AON_RTC_COMB"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH15SSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH15SSEL_EV {
        UDMACH15SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH15SSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH15SSEL_EV) -> u32 {
        UDMACH15SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH16BSEL_EV(u8);
impl UDMACH16BSEL_EV {
    #[doc = "SSI1 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE."]
    pub const SSI1_RX_DMABREQ: Self = Self(0x2c);
}
impl UDMACH16BSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH16BSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH16BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x2c => f.write_str("SSI1_RX_DMABREQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH16BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x2c => defmt::write!(f, "SSI1_RX_DMABREQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH16BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH16BSEL_EV {
        UDMACH16BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH16BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH16BSEL_EV) -> u8 {
        UDMACH16BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH16SSEL_EV(u8);
impl UDMACH16SSEL_EV {
    #[doc = "SSI1 RX DMA single request, controlled by SSI0:DMACR.RXDMAE."]
    pub const SSI1_RX_DMASREQ: Self = Self(0x2d);
}
impl UDMACH16SSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH16SSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH16SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x2d => f.write_str("SSI1_RX_DMASREQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH16SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x2d => defmt::write!(f, "SSI1_RX_DMASREQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH16SSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH16SSEL_EV {
        UDMACH16SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH16SSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH16SSEL_EV) -> u8 {
        UDMACH16SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH17BSEL_EV(u8);
impl UDMACH17BSEL_EV {
    #[doc = "SSI1 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE."]
    pub const SSI1_TX_DMABREQ: Self = Self(0x2e);
}
impl UDMACH17BSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH17BSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH17BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x2e => f.write_str("SSI1_TX_DMABREQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH17BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x2e => defmt::write!(f, "SSI1_TX_DMABREQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH17BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH17BSEL_EV {
        UDMACH17BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH17BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH17BSEL_EV) -> u8 {
        UDMACH17BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH17SSEL_EV(u8);
impl UDMACH17SSEL_EV {
    #[doc = "SSI1 TX DMA single request, controlled by SSI0:DMACR.TXDMAE."]
    pub const SSI1_TX_DMASREQ: Self = Self(0x2f);
}
impl UDMACH17SSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH17SSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH17SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x2f => f.write_str("SSI1_TX_DMASREQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH17SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x2f => defmt::write!(f, "SSI1_TX_DMASREQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH17SSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH17SSEL_EV {
        UDMACH17SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH17SSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH17SSEL_EV) -> u8 {
        UDMACH17SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH18BSEL_EV(u32);
impl UDMACH18BSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH18BSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH18BSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH18BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH18BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH18BSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH18BSEL_EV {
        UDMACH18BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH18BSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH18BSEL_EV) -> u32 {
        UDMACH18BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH18SSEL_EV(u32);
impl UDMACH18SSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH18SSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH18SSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH18SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH18SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH18SSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH18SSEL_EV {
        UDMACH18SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH18SSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH18SSEL_EV) -> u32 {
        UDMACH18SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH19BSEL_EV(u32);
impl UDMACH19BSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH19BSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH19BSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH19BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH19BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH19BSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH19BSEL_EV {
        UDMACH19BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH19BSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH19BSEL_EV) -> u32 {
        UDMACH19BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH19SSEL_EV(u32);
impl UDMACH19SSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH19SSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH19SSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH19SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH19SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH19SSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH19SSEL_EV {
        UDMACH19SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH19SSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH19SSEL_EV) -> u32 {
        UDMACH19SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH1BSEL_EV(u8);
impl UDMACH1BSEL_EV {
    #[doc = "UART0 RX DMA burst request, controlled by UART0:DMACTL.RXDMAE."]
    pub const UART0_RX_DMABREQ: Self = Self(0x30);
}
impl UDMACH1BSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH1BSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH1BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x30 => f.write_str("UART0_RX_DMABREQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH1BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x30 => defmt::write!(f, "UART0_RX_DMABREQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH1BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH1BSEL_EV {
        UDMACH1BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH1BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH1BSEL_EV) -> u8 {
        UDMACH1BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH1SSEL_EV(u8);
impl UDMACH1SSEL_EV {
    #[doc = "UART0 RX DMA single request, controlled by UART0:DMACTL.RXDMAE."]
    pub const UART0_RX_DMASREQ: Self = Self(0x31);
}
impl UDMACH1SSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH1SSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH1SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x31 => f.write_str("UART0_RX_DMASREQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH1SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x31 => defmt::write!(f, "UART0_RX_DMASREQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH1SSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH1SSEL_EV {
        UDMACH1SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH1SSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH1SSEL_EV) -> u8 {
        UDMACH1SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH20BSEL_EV(u32);
impl UDMACH20BSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH20BSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH20BSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH20BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH20BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH20BSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH20BSEL_EV {
        UDMACH20BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH20BSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH20BSEL_EV) -> u32 {
        UDMACH20BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH20SSEL_EV(u32);
impl UDMACH20SSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH20SSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH20SSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH20SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH20SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH20SSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH20SSEL_EV {
        UDMACH20SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH20SSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH20SSEL_EV) -> u32 {
        UDMACH20SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH21BSEL_EV(u8);
impl UDMACH21BSEL_EV {
    #[doc = "Software event 0, triggered by SWEV.SWEV0."]
    pub const SWEV0: Self = Self(0x64);
}
impl UDMACH21BSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH21BSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH21BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x64 => f.write_str("SWEV0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH21BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x64 => defmt::write!(f, "SWEV0"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH21BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH21BSEL_EV {
        UDMACH21BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH21BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH21BSEL_EV) -> u8 {
        UDMACH21BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH21SSEL_EV(u8);
impl UDMACH21SSEL_EV {
    #[doc = "Software event 0, triggered by SWEV.SWEV0."]
    pub const SWEV0: Self = Self(0x64);
}
impl UDMACH21SSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH21SSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH21SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x64 => f.write_str("SWEV0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH21SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x64 => defmt::write!(f, "SWEV0"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH21SSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH21SSEL_EV {
        UDMACH21SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH21SSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH21SSEL_EV) -> u8 {
        UDMACH21SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH22BSEL_EV(u8);
impl UDMACH22BSEL_EV {
    #[doc = "Software event 1, triggered by SWEV.SWEV1."]
    pub const SWEV1: Self = Self(0x65);
}
impl UDMACH22BSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH22BSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH22BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x65 => f.write_str("SWEV1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH22BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x65 => defmt::write!(f, "SWEV1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH22BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH22BSEL_EV {
        UDMACH22BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH22BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH22BSEL_EV) -> u8 {
        UDMACH22BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH22SSEL_EV(u8);
impl UDMACH22SSEL_EV {
    #[doc = "Software event 1, triggered by SWEV.SWEV1."]
    pub const SWEV1: Self = Self(0x65);
}
impl UDMACH22SSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH22SSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH22SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x65 => f.write_str("SWEV1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH22SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x65 => defmt::write!(f, "SWEV1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH22SSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH22SSEL_EV {
        UDMACH22SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH22SSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH22SSEL_EV) -> u8 {
        UDMACH22SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH23BSEL_EV(u8);
impl UDMACH23BSEL_EV {
    #[doc = "Software event 2, triggered by SWEV.SWEV2."]
    pub const SWEV2: Self = Self(0x66);
}
impl UDMACH23BSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH23BSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH23BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x66 => f.write_str("SWEV2"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH23BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x66 => defmt::write!(f, "SWEV2"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH23BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH23BSEL_EV {
        UDMACH23BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH23BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH23BSEL_EV) -> u8 {
        UDMACH23BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH23SSEL_EV(u8);
impl UDMACH23SSEL_EV {
    #[doc = "Software event 2, triggered by SWEV.SWEV2."]
    pub const SWEV2: Self = Self(0x66);
}
impl UDMACH23SSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH23SSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH23SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x66 => f.write_str("SWEV2"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH23SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x66 => defmt::write!(f, "SWEV2"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH23SSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH23SSEL_EV {
        UDMACH23SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH23SSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH23SSEL_EV) -> u8 {
        UDMACH23SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH24BSEL_EV(u8);
impl UDMACH24BSEL_EV {
    #[doc = "Software event 3, triggered by SWEV.SWEV3."]
    pub const SWEV3: Self = Self(0x67);
}
impl UDMACH24BSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH24BSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH24BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x67 => f.write_str("SWEV3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH24BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x67 => defmt::write!(f, "SWEV3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH24BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH24BSEL_EV {
        UDMACH24BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH24BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH24BSEL_EV) -> u8 {
        UDMACH24BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH24SSEL_EV(u8);
impl UDMACH24SSEL_EV {
    #[doc = "Software event 3, triggered by SWEV.SWEV3."]
    pub const SWEV3: Self = Self(0x67);
}
impl UDMACH24SSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH24SSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH24SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x67 => f.write_str("SWEV3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH24SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x67 => defmt::write!(f, "SWEV3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH24SSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH24SSEL_EV {
        UDMACH24SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH24SSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH24SSEL_EV) -> u8 {
        UDMACH24SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH25BSEL_EV(u32);
impl UDMACH25BSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH25BSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH25BSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH25BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH25BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH25BSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH25BSEL_EV {
        UDMACH25BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH25BSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH25BSEL_EV) -> u32 {
        UDMACH25BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH25SSEL_EV(u32);
impl UDMACH25SSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH25SSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH25SSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH25SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH25SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH25SSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH25SSEL_EV {
        UDMACH25SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH25SSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH25SSEL_EV) -> u32 {
        UDMACH25SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH26BSEL_EV(u32);
impl UDMACH26BSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH26BSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH26BSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH26BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH26BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH26BSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH26BSEL_EV {
        UDMACH26BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH26BSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH26BSEL_EV) -> u32 {
        UDMACH26BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH26SSEL_EV(u32);
impl UDMACH26SSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH26SSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH26SSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH26SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH26SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH26SSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH26SSEL_EV {
        UDMACH26SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH26SSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH26SSEL_EV) -> u32 {
        UDMACH26SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH27BSEL_EV(u32);
impl UDMACH27BSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH27BSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH27BSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH27BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH27BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH27BSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH27BSEL_EV {
        UDMACH27BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH27BSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH27BSEL_EV) -> u32 {
        UDMACH27BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH27SSEL_EV(u32);
impl UDMACH27SSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH27SSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH27SSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH27SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH27SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH27SSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH27SSEL_EV {
        UDMACH27SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH27SSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH27SSEL_EV) -> u32 {
        UDMACH27SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH28BSEL_EV(u32);
impl UDMACH28BSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH28BSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH28BSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH28BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH28BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH28BSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH28BSEL_EV {
        UDMACH28BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH28BSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH28BSEL_EV) -> u32 {
        UDMACH28BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH28SSEL_EV(u32);
impl UDMACH28SSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH28SSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH28SSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH28SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH28SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH28SSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH28SSEL_EV {
        UDMACH28SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH28SSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH28SSEL_EV) -> u32 {
        UDMACH28SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH29BSEL_EV(u32);
impl UDMACH29BSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH29BSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH29BSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH29BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH29BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH29BSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH29BSEL_EV {
        UDMACH29BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH29BSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH29BSEL_EV) -> u32 {
        UDMACH29BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH29SSEL_EV(u32);
impl UDMACH29SSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH29SSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH29SSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH29SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH29SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH29SSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH29SSEL_EV {
        UDMACH29SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH29SSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH29SSEL_EV) -> u32 {
        UDMACH29SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH2BSEL_EV(u8);
impl UDMACH2BSEL_EV {
    #[doc = "UART0 TX DMA burst request, controlled by UART0:DMACTL.TXDMAE."]
    pub const UART0_TX_DMABREQ: Self = Self(0x32);
}
impl UDMACH2BSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH2BSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH2BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x32 => f.write_str("UART0_TX_DMABREQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH2BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x32 => defmt::write!(f, "UART0_TX_DMABREQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH2BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH2BSEL_EV {
        UDMACH2BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH2BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH2BSEL_EV) -> u8 {
        UDMACH2BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH2SSEL_EV(u8);
impl UDMACH2SSEL_EV {
    #[doc = "UART0 TX DMA single request, controlled by UART0:DMACTL.TXDMAE."]
    pub const UART0_TX_DMASREQ: Self = Self(0x33);
}
impl UDMACH2SSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH2SSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH2SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x33 => f.write_str("UART0_TX_DMASREQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH2SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x33 => defmt::write!(f, "UART0_TX_DMASREQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH2SSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH2SSEL_EV {
        UDMACH2SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH2SSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH2SSEL_EV) -> u8 {
        UDMACH2SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH30BSEL_EV(u32);
impl UDMACH30BSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH30BSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH30BSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH30BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH30BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH30BSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH30BSEL_EV {
        UDMACH30BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH30BSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH30BSEL_EV) -> u32 {
        UDMACH30BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH30SSEL_EV(u32);
impl UDMACH30SSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH30SSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH30SSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH30SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH30SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH30SSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH30SSEL_EV {
        UDMACH30SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH30SSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH30SSEL_EV) -> u32 {
        UDMACH30SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH31BSEL_EV(u32);
impl UDMACH31BSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH31BSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH31BSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH31BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH31BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH31BSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH31BSEL_EV {
        UDMACH31BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH31BSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH31BSEL_EV) -> u32 {
        UDMACH31BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH31SSEL_EV(u32);
impl UDMACH31SSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
}
impl UDMACH31SSEL_EV {
    pub const fn from_bits(val: u32) -> UDMACH31SSEL_EV {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH31SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH31SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UDMACH31SSEL_EV {
    #[inline(always)]
    fn from(val: u32) -> UDMACH31SSEL_EV {
        UDMACH31SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH31SSEL_EV> for u32 {
    #[inline(always)]
    fn from(val: UDMACH31SSEL_EV) -> u32 {
        UDMACH31SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH3BSEL_EV(u8);
impl UDMACH3BSEL_EV {
    #[doc = "SSI0 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE."]
    pub const SSI0_RX_DMABREQ: Self = Self(0x28);
}
impl UDMACH3BSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH3BSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH3BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x28 => f.write_str("SSI0_RX_DMABREQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH3BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x28 => defmt::write!(f, "SSI0_RX_DMABREQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH3BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH3BSEL_EV {
        UDMACH3BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH3BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH3BSEL_EV) -> u8 {
        UDMACH3BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH3SSEL_EV(u8);
impl UDMACH3SSEL_EV {
    #[doc = "SSI0 RX DMA single request, controlled by SSI0:DMACR.RXDMAE."]
    pub const SSI0_RX_DMASREQ: Self = Self(0x29);
}
impl UDMACH3SSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH3SSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH3SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x29 => f.write_str("SSI0_RX_DMASREQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH3SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x29 => defmt::write!(f, "SSI0_RX_DMASREQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH3SSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH3SSEL_EV {
        UDMACH3SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH3SSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH3SSEL_EV) -> u8 {
        UDMACH3SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH4BSEL_EV(u8);
impl UDMACH4BSEL_EV {
    #[doc = "SSI0 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE."]
    pub const SSI0_TX_DMABREQ: Self = Self(0x2a);
}
impl UDMACH4BSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH4BSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH4BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x2a => f.write_str("SSI0_TX_DMABREQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH4BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x2a => defmt::write!(f, "SSI0_TX_DMABREQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH4BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH4BSEL_EV {
        UDMACH4BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH4BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH4BSEL_EV) -> u8 {
        UDMACH4BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH4SSEL_EV(u8);
impl UDMACH4SSEL_EV {
    #[doc = "SSI0 TX DMA single request, controlled by SSI0:DMACR.TXDMAE."]
    pub const SSI0_TX_DMASREQ: Self = Self(0x2b);
}
impl UDMACH4SSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH4SSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH4SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x2b => f.write_str("SSI0_TX_DMASREQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH4SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x2b => defmt::write!(f, "SSI0_TX_DMASREQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH4SSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH4SSEL_EV {
        UDMACH4SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH4SSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH4SSEL_EV) -> u8 {
        UDMACH4SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH7BSEL_EV(u8);
impl UDMACH7BSEL_EV {
    #[doc = "DMA burst request event from AUX, configured by AUX_EVCTL:DMACTL."]
    pub const AUX_DMABREQ: Self = Self(0x76);
}
impl UDMACH7BSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH7BSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH7BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x76 => f.write_str("AUX_DMABREQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH7BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x76 => defmt::write!(f, "AUX_DMABREQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH7BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH7BSEL_EV {
        UDMACH7BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH7BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH7BSEL_EV) -> u8 {
        UDMACH7BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH7SSEL_EV(u8);
impl UDMACH7SSEL_EV {
    #[doc = "DMA single request event from AUX, configured by AUX_EVCTL:DMACTL."]
    pub const AUX_DMASREQ: Self = Self(0x75);
}
impl UDMACH7SSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH7SSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH7SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x75 => f.write_str("AUX_DMASREQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH7SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x75 => defmt::write!(f, "AUX_DMASREQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH7SSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH7SSEL_EV {
        UDMACH7SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH7SSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH7SSEL_EV) -> u8 {
        UDMACH7SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH8BSEL_EV(u8);
impl UDMACH8BSEL_EV {
    #[doc = "DMA sofware trigger from AUX, triggered by AUX_EVCTL:DMASWREQ.START."]
    pub const AUX_SW_DMABREQ: Self = Self(0x74);
}
impl UDMACH8BSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH8BSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH8BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x74 => f.write_str("AUX_SW_DMABREQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH8BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x74 => defmt::write!(f, "AUX_SW_DMABREQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH8BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH8BSEL_EV {
        UDMACH8BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH8BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH8BSEL_EV) -> u8 {
        UDMACH8BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH8SSEL_EV(u8);
impl UDMACH8SSEL_EV {
    #[doc = "DMA sofware trigger from AUX, triggered by AUX_EVCTL:DMASWREQ.START."]
    pub const AUX_SW_DMABREQ: Self = Self(0x74);
}
impl UDMACH8SSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH8SSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH8SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x74 => f.write_str("AUX_SW_DMABREQ"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH8SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x74 => defmt::write!(f, "AUX_SW_DMABREQ"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH8SSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH8SSEL_EV {
        UDMACH8SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH8SSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH8SSEL_EV) -> u8 {
        UDMACH8SSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH9BSEL_EV(u8);
impl UDMACH9BSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV."]
    pub const GPT0A_DMABREQ: Self = Self(0x4d);
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV."]
    pub const GPT0B_DMABREQ: Self = Self(0x4e);
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV."]
    pub const GPT1A_DMABREQ: Self = Self(0x4f);
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV."]
    pub const GPT1B_DMABREQ: Self = Self(0x50);
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV."]
    pub const GPT2A_DMABREQ: Self = Self(0x51);
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV."]
    pub const GPT2B_DMABREQ: Self = Self(0x52);
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV."]
    pub const GPT3A_DMABREQ: Self = Self(0x53);
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV."]
    pub const GPT3B_DMABREQ: Self = Self(0x54);
    #[doc = "Always asserted."]
    pub const ALWAYS_ACTIVE: Self = Self(0x79);
}
impl UDMACH9BSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH9BSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH9BSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            0x4d => f.write_str("GPT0A_DMABREQ"),
            0x4e => f.write_str("GPT0B_DMABREQ"),
            0x4f => f.write_str("GPT1A_DMABREQ"),
            0x50 => f.write_str("GPT1B_DMABREQ"),
            0x51 => f.write_str("GPT2A_DMABREQ"),
            0x52 => f.write_str("GPT2B_DMABREQ"),
            0x53 => f.write_str("GPT3A_DMABREQ"),
            0x54 => f.write_str("GPT3B_DMABREQ"),
            0x79 => f.write_str("ALWAYS_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH9BSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            0x4d => defmt::write!(f, "GPT0A_DMABREQ"),
            0x4e => defmt::write!(f, "GPT0B_DMABREQ"),
            0x4f => defmt::write!(f, "GPT1A_DMABREQ"),
            0x50 => defmt::write!(f, "GPT1B_DMABREQ"),
            0x51 => defmt::write!(f, "GPT2A_DMABREQ"),
            0x52 => defmt::write!(f, "GPT2B_DMABREQ"),
            0x53 => defmt::write!(f, "GPT3A_DMABREQ"),
            0x54 => defmt::write!(f, "GPT3B_DMABREQ"),
            0x79 => defmt::write!(f, "ALWAYS_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH9BSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH9BSEL_EV {
        UDMACH9BSEL_EV::from_bits(val)
    }
}
impl From<UDMACH9BSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH9BSEL_EV) -> u8 {
        UDMACH9BSEL_EV::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UDMACH9SSEL_EV(u8);
impl UDMACH9SSEL_EV {
    #[doc = "Always inactive."]
    pub const NONE: Self = Self(0x0);
    #[doc = "Not used tied to 0."]
    pub const TIE_LOW: Self = Self(0x45);
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV."]
    pub const GPT0A_DMABREQ: Self = Self(0x4d);
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV."]
    pub const GPT0B_DMABREQ: Self = Self(0x4e);
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV."]
    pub const GPT1A_DMABREQ: Self = Self(0x4f);
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV."]
    pub const GPT1B_DMABREQ: Self = Self(0x50);
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV."]
    pub const GPT2A_DMABREQ: Self = Self(0x51);
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV."]
    pub const GPT2B_DMABREQ: Self = Self(0x52);
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV."]
    pub const GPT3A_DMABREQ: Self = Self(0x53);
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV."]
    pub const GPT3B_DMABREQ: Self = Self(0x54);
    #[doc = "Always asserted."]
    pub const ALWAYS_ACTIVE: Self = Self(0x79);
}
impl UDMACH9SSEL_EV {
    pub const fn from_bits(val: u8) -> UDMACH9SSEL_EV {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for UDMACH9SSEL_EV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONE"),
            0x45 => f.write_str("TIE_LOW"),
            0x4d => f.write_str("GPT0A_DMABREQ"),
            0x4e => f.write_str("GPT0B_DMABREQ"),
            0x4f => f.write_str("GPT1A_DMABREQ"),
            0x50 => f.write_str("GPT1B_DMABREQ"),
            0x51 => f.write_str("GPT2A_DMABREQ"),
            0x52 => f.write_str("GPT2B_DMABREQ"),
            0x53 => f.write_str("GPT3A_DMABREQ"),
            0x54 => f.write_str("GPT3B_DMABREQ"),
            0x79 => f.write_str("ALWAYS_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH9SSEL_EV {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONE"),
            0x45 => defmt::write!(f, "TIE_LOW"),
            0x4d => defmt::write!(f, "GPT0A_DMABREQ"),
            0x4e => defmt::write!(f, "GPT0B_DMABREQ"),
            0x4f => defmt::write!(f, "GPT1A_DMABREQ"),
            0x50 => defmt::write!(f, "GPT1B_DMABREQ"),
            0x51 => defmt::write!(f, "GPT2A_DMABREQ"),
            0x52 => defmt::write!(f, "GPT2B_DMABREQ"),
            0x53 => defmt::write!(f, "GPT3A_DMABREQ"),
            0x54 => defmt::write!(f, "GPT3B_DMABREQ"),
            0x79 => defmt::write!(f, "ALWAYS_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for UDMACH9SSEL_EV {
    #[inline(always)]
    fn from(val: u8) -> UDMACH9SSEL_EV {
        UDMACH9SSEL_EV::from_bits(val)
    }
}
impl From<UDMACH9SSEL_EV> for u8 {
    #[inline(always)]
    fn from(val: UDMACH9SSEL_EV) -> u8 {
        UDMACH9SSEL_EV::to_bits(val)
    }
}
