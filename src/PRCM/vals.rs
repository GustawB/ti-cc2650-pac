#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AVAIL(u8);
impl AVAIL {
    #[doc = "Mode 0 permitted."]
    pub const MODE0: Self = Self(0x01);
    #[doc = "Mode 1 permitted."]
    pub const MODE1: Self = Self(0x02);
    #[doc = "Mode 2 permitted."]
    pub const MODE2: Self = Self(0x04);
    #[doc = "Mode 3 permitted."]
    pub const MODE3: Self = Self(0x08);
    #[doc = "Mode 4 permitted."]
    pub const MODE4: Self = Self(0x10);
    #[doc = "Mode 5 permitted."]
    pub const MODE5: Self = Self(0x20);
    #[doc = "Mode 6 permitted."]
    pub const MODE6: Self = Self(0x40);
    #[doc = "Mode 7 permitted."]
    pub const MODE7: Self = Self(0x80);
}
impl AVAIL {
    pub const fn from_bits(val: u8) -> AVAIL {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for AVAIL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("MODE0"),
            0x02 => f.write_str("MODE1"),
            0x04 => f.write_str("MODE2"),
            0x08 => f.write_str("MODE3"),
            0x10 => f.write_str("MODE4"),
            0x20 => f.write_str("MODE5"),
            0x40 => f.write_str("MODE6"),
            0x80 => f.write_str("MODE7"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AVAIL {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "MODE0"),
            0x02 => defmt::write!(f, "MODE1"),
            0x04 => defmt::write!(f, "MODE2"),
            0x08 => defmt::write!(f, "MODE3"),
            0x10 => defmt::write!(f, "MODE4"),
            0x20 => defmt::write!(f, "MODE5"),
            0x40 => defmt::write!(f, "MODE6"),
            0x80 => defmt::write!(f, "MODE7"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for AVAIL {
    #[inline(always)]
    fn from(val: u8) -> AVAIL {
        AVAIL::from_bits(val)
    }
}
impl From<AVAIL> for u8 {
    #[inline(always)]
    fn from(val: AVAIL) -> u8 {
        AVAIL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CPUCLKDIV_RATIO {
    #[doc = "Internal. Only to be used through TI provided API."]
    DIV1 = 0x0,
    #[doc = "Internal. Only to be used through TI provided API."]
    DIV2 = 0x01,
}
impl CPUCLKDIV_RATIO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CPUCLKDIV_RATIO {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CPUCLKDIV_RATIO {
    #[inline(always)]
    fn from(val: u8) -> CPUCLKDIV_RATIO {
        CPUCLKDIV_RATIO::from_bits(val)
    }
}
impl From<CPUCLKDIV_RATIO> for u8 {
    #[inline(always)]
    fn from(val: CPUCLKDIV_RATIO) -> u8 {
        CPUCLKDIV_RATIO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CURR {
    #[doc = "Select Mode 0."]
    MODE0 = 0x0,
    #[doc = "Select Mode 1."]
    MODE1 = 0x01,
    #[doc = "Select Mode 2."]
    MODE2 = 0x02,
    #[doc = "Select Mode 3."]
    MODE3 = 0x03,
    #[doc = "Select Mode 4."]
    MODE4 = 0x04,
    #[doc = "Select Mode 5."]
    MODE5 = 0x05,
    #[doc = "Select Mode 6."]
    MODE6 = 0x06,
    #[doc = "Select Mode 7."]
    MODE7 = 0x07,
}
impl CURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CURR {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CURR {
    #[inline(always)]
    fn from(val: u8) -> CURR {
        CURR::from_bits(val)
    }
}
impl From<CURR> for u8 {
    #[inline(always)]
    fn from(val: CURR) -> u8 {
        CURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPTCLKDIV_RATIO {
    #[doc = "Divide by 1."]
    DIV1 = 0x0,
    #[doc = "Divide by 2."]
    DIV2 = 0x01,
    #[doc = "Divide by 4."]
    DIV4 = 0x02,
    #[doc = "Divide by 8."]
    DIV8 = 0x03,
    #[doc = "Divide by 16."]
    DIV16 = 0x04,
    #[doc = "Divide by 32."]
    DIV32 = 0x05,
    #[doc = "Divide by 64."]
    DIV64 = 0x06,
    #[doc = "Divide by 128."]
    DIV128 = 0x07,
    #[doc = "Divide by 256."]
    DIV256 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GPTCLKDIV_RATIO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPTCLKDIV_RATIO {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPTCLKDIV_RATIO {
    #[inline(always)]
    fn from(val: u8) -> GPTCLKDIV_RATIO {
        GPTCLKDIV_RATIO::from_bits(val)
    }
}
impl From<GPTCLKDIV_RATIO> for u8 {
    #[inline(always)]
    fn from(val: GPTCLKDIV_RATIO) -> u8 {
        GPTCLKDIV_RATIO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPTCLKGDS_CLK_EN {
    _RESERVED_0 = 0x0,
    #[doc = "Enable clock for GPT0."]
    GPT0 = 0x01,
    #[doc = "Enable clock for GPT1."]
    GPT1 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Enable clock for GPT2."]
    GPT2 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Enable clock for GPT3."]
    GPT3 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GPTCLKGDS_CLK_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPTCLKGDS_CLK_EN {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPTCLKGDS_CLK_EN {
    #[inline(always)]
    fn from(val: u8) -> GPTCLKGDS_CLK_EN {
        GPTCLKGDS_CLK_EN::from_bits(val)
    }
}
impl From<GPTCLKGDS_CLK_EN> for u8 {
    #[inline(always)]
    fn from(val: GPTCLKGDS_CLK_EN) -> u8 {
        GPTCLKGDS_CLK_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPTCLKGR_CLK_EN {
    _RESERVED_0 = 0x0,
    #[doc = "Enable clock for GPT0."]
    GPT0 = 0x01,
    #[doc = "Enable clock for GPT1."]
    GPT1 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Enable clock for GPT2."]
    GPT2 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Enable clock for GPT3."]
    GPT3 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GPTCLKGR_CLK_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPTCLKGR_CLK_EN {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPTCLKGR_CLK_EN {
    #[inline(always)]
    fn from(val: u8) -> GPTCLKGR_CLK_EN {
        GPTCLKGR_CLK_EN::from_bits(val)
    }
}
impl From<GPTCLKGR_CLK_EN> for u8 {
    #[inline(always)]
    fn from(val: GPTCLKGR_CLK_EN) -> u8 {
        GPTCLKGR_CLK_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPTCLKGS_CLK_EN {
    _RESERVED_0 = 0x0,
    #[doc = "Enable clock for GPT0."]
    GPT0 = 0x01,
    #[doc = "Enable clock for GPT1."]
    GPT1 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Enable clock for GPT2."]
    GPT2 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Enable clock for GPT3."]
    GPT3 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GPTCLKGS_CLK_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPTCLKGS_CLK_EN {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPTCLKGS_CLK_EN {
    #[inline(always)]
    fn from(val: u8) -> GPTCLKGS_CLK_EN {
        GPTCLKGS_CLK_EN::from_bits(val)
    }
}
impl From<GPTCLKGS_CLK_EN> for u8 {
    #[inline(always)]
    fn from(val: GPTCLKGS_CLK_EN) -> u8 {
        GPTCLKGS_CLK_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INFRCLKDIVDS_RATIO {
    #[doc = "Divide by 1."]
    DIV1 = 0x0,
    #[doc = "Divide by 2."]
    DIV2 = 0x01,
    #[doc = "Divide by 8."]
    DIV8 = 0x02,
    #[doc = "Divide by 32."]
    DIV32 = 0x03,
}
impl INFRCLKDIVDS_RATIO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INFRCLKDIVDS_RATIO {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INFRCLKDIVDS_RATIO {
    #[inline(always)]
    fn from(val: u8) -> INFRCLKDIVDS_RATIO {
        INFRCLKDIVDS_RATIO::from_bits(val)
    }
}
impl From<INFRCLKDIVDS_RATIO> for u8 {
    #[inline(always)]
    fn from(val: INFRCLKDIVDS_RATIO) -> u8 {
        INFRCLKDIVDS_RATIO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INFRCLKDIVR_RATIO {
    #[doc = "Divide by 1."]
    DIV1 = 0x0,
    #[doc = "Divide by 2."]
    DIV2 = 0x01,
    #[doc = "Divide by 8."]
    DIV8 = 0x02,
    #[doc = "Divide by 32."]
    DIV32 = 0x03,
}
impl INFRCLKDIVR_RATIO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INFRCLKDIVR_RATIO {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INFRCLKDIVR_RATIO {
    #[inline(always)]
    fn from(val: u8) -> INFRCLKDIVR_RATIO {
        INFRCLKDIVR_RATIO::from_bits(val)
    }
}
impl From<INFRCLKDIVR_RATIO> for u8 {
    #[inline(always)]
    fn from(val: INFRCLKDIVR_RATIO) -> u8 {
        INFRCLKDIVR_RATIO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INFRCLKDIVS_RATIO {
    #[doc = "Divide by 1."]
    DIV1 = 0x0,
    #[doc = "Divide by 2."]
    DIV2 = 0x01,
    #[doc = "Divide by 8."]
    DIV8 = 0x02,
    #[doc = "Divide by 32."]
    DIV32 = 0x03,
}
impl INFRCLKDIVS_RATIO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INFRCLKDIVS_RATIO {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INFRCLKDIVS_RATIO {
    #[inline(always)]
    fn from(val: u8) -> INFRCLKDIVS_RATIO {
        INFRCLKDIVS_RATIO::from_bits(val)
    }
}
impl From<INFRCLKDIVS_RATIO> for u8 {
    #[inline(always)]
    fn from(val: INFRCLKDIVS_RATIO) -> u8 {
        INFRCLKDIVS_RATIO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SSICLKGDS_CLK_EN {
    _RESERVED_0 = 0x0,
    #[doc = "Enable clock for SSI0."]
    SSI0 = 0x01,
    #[doc = "Enable clock for SSI1."]
    SSI1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SSICLKGDS_CLK_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SSICLKGDS_CLK_EN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SSICLKGDS_CLK_EN {
    #[inline(always)]
    fn from(val: u8) -> SSICLKGDS_CLK_EN {
        SSICLKGDS_CLK_EN::from_bits(val)
    }
}
impl From<SSICLKGDS_CLK_EN> for u8 {
    #[inline(always)]
    fn from(val: SSICLKGDS_CLK_EN) -> u8 {
        SSICLKGDS_CLK_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SSICLKGR_CLK_EN {
    _RESERVED_0 = 0x0,
    #[doc = "Enable clock for SSI0."]
    SSI0 = 0x01,
    #[doc = "Enable clock for SSI1."]
    SSI1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SSICLKGR_CLK_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SSICLKGR_CLK_EN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SSICLKGR_CLK_EN {
    #[inline(always)]
    fn from(val: u8) -> SSICLKGR_CLK_EN {
        SSICLKGR_CLK_EN::from_bits(val)
    }
}
impl From<SSICLKGR_CLK_EN> for u8 {
    #[inline(always)]
    fn from(val: SSICLKGR_CLK_EN) -> u8 {
        SSICLKGR_CLK_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SSICLKGS_CLK_EN {
    _RESERVED_0 = 0x0,
    #[doc = "Enable clock for SSI0."]
    SSI0 = 0x01,
    #[doc = "Enable clock for SSI1."]
    SSI1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SSICLKGS_CLK_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SSICLKGS_CLK_EN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SSICLKGS_CLK_EN {
    #[inline(always)]
    fn from(val: u8) -> SSICLKGS_CLK_EN {
        SSICLKGS_CLK_EN::from_bits(val)
    }
}
impl From<SSICLKGS_CLK_EN> for u8 {
    #[inline(always)]
    fn from(val: SSICLKGS_CLK_EN) -> u8 {
        SSICLKGS_CLK_EN::to_bits(val)
    }
}
