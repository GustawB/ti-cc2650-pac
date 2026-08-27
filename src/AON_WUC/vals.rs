#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AUXCLK_PWR_DWN_SRC {
    #[doc = "No clock in Powerdown."]
    NONE = 0x0,
    #[doc = "Use SCLK_LF in Powerdown."]
    SCLK_LF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl AUXCLK_PWR_DWN_SRC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AUXCLK_PWR_DWN_SRC {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AUXCLK_PWR_DWN_SRC {
    #[inline(always)]
    fn from(val: u8) -> AUXCLK_PWR_DWN_SRC {
        AUXCLK_PWR_DWN_SRC::from_bits(val)
    }
}
impl From<AUXCLK_PWR_DWN_SRC> for u8 {
    #[inline(always)]
    fn from(val: AUXCLK_PWR_DWN_SRC) -> u8 {
        AUXCLK_PWR_DWN_SRC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MCUCLK_PWR_DWN_SRC {
    #[doc = "No clock in Powerdown."]
    NONE = 0x0,
    #[doc = "Use SCLK_LF in Powerdown."]
    SCLK_LF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl MCUCLK_PWR_DWN_SRC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MCUCLK_PWR_DWN_SRC {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MCUCLK_PWR_DWN_SRC {
    #[inline(always)]
    fn from(val: u8) -> MCUCLK_PWR_DWN_SRC {
        MCUCLK_PWR_DWN_SRC::from_bits(val)
    }
}
impl From<MCUCLK_PWR_DWN_SRC> for u8 {
    #[inline(always)]
    fn from(val: MCUCLK_PWR_DWN_SRC) -> u8 {
        MCUCLK_PWR_DWN_SRC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SCLK_HF_DIV {
    #[doc = "Divide by 2."]
    DIV2 = 0x0,
    #[doc = "Divide by 4."]
    DIV4 = 0x01,
    #[doc = "Divide by 8."]
    DIV8 = 0x02,
    #[doc = "Divide by 16."]
    DIV16 = 0x03,
    #[doc = "Divide by 32."]
    DIV32 = 0x04,
    #[doc = "Divide by 64."]
    DIV64 = 0x05,
    #[doc = "Divide by 128."]
    DIV128 = 0x06,
    #[doc = "Divide by 256."]
    DIV256 = 0x07,
}
impl SCLK_HF_DIV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SCLK_HF_DIV {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SCLK_HF_DIV {
    #[inline(always)]
    fn from(val: u8) -> SCLK_HF_DIV {
        SCLK_HF_DIV::from_bits(val)
    }
}
impl From<SCLK_HF_DIV> for u8 {
    #[inline(always)]
    fn from(val: SCLK_HF_DIV) -> u8 {
        SCLK_HF_DIV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRAM_RET_EN {
    #[doc = "Retention is disabled."]
    RET_NONE = 0x0,
    #[doc = "Retention on for SRAM:BANK0."]
    RET_LEVEL1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Retention on for SRAM:BANK0 and SRAM:BANK1."]
    RET_LEVEL2 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Retention on for SRAM:BANK0, SRAM:BANK1 and SRAM:BANK2."]
    RET_LEVEL3 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Retention on for all banks (SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2 and SRAM:BANK3)."]
    RET_FULL = 0x0f,
}
impl SRAM_RET_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRAM_RET_EN {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRAM_RET_EN {
    #[inline(always)]
    fn from(val: u8) -> SRAM_RET_EN {
        SRAM_RET_EN::from_bits(val)
    }
}
impl From<SRAM_RET_EN> for u8 {
    #[inline(always)]
    fn from(val: SRAM_RET_EN) -> u8 {
        SRAM_RET_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRC {
    _RESERVED_0 = 0x0,
    #[doc = "HF Clock (SCLK_HF)."]
    SCLK_HF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "LF Clock (SCLK_LF)."]
    SCLK_LF = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SRC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRC {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRC {
    #[inline(always)]
    fn from(val: u8) -> SRC {
        SRC::from_bits(val)
    }
}
impl From<SRC> for u8 {
    #[inline(always)]
    fn from(val: SRC) -> u8 {
        SRC::to_bits(val)
    }
}
