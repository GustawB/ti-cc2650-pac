#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AD0 {
    #[doc = "Not in use (disabled)."]
    DIS = 0x0,
    #[doc = "Input mode."]
    IN = 0x01,
    #[doc = "Output mode."]
    OUT = 0x02,
    _RESERVED_3 = 0x03,
}
impl AD0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AD0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AD0 {
    #[inline(always)]
    fn from(val: u8) -> AD0 {
        AD0::from_bits(val)
    }
}
impl From<AD0> for u8 {
    #[inline(always)]
    fn from(val: AD0) -> u8 {
        AD0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AD1 {
    #[doc = "Not in use (disabled)."]
    DIS = 0x0,
    #[doc = "Input mode."]
    IN = 0x01,
    #[doc = "Output mode."]
    OUT = 0x02,
    _RESERVED_3 = 0x03,
}
impl AD1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AD1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AD1 {
    #[inline(always)]
    fn from(val: u8) -> AD1 {
        AD1::from_bits(val)
    }
}
impl From<AD1> for u8 {
    #[inline(always)]
    fn from(val: AD1) -> u8 {
        AD1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MEM_LEN_24 {
    #[doc = "16-bit (one 16 bit access per sample)."]
    _16BIT = 0x0,
    #[doc = "24-bit (one 8 bit and one 16 bit locked access per sample)."]
    _24BIT = 0x01,
}
impl MEM_LEN_24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MEM_LEN_24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MEM_LEN_24 {
    #[inline(always)]
    fn from(val: u8) -> MEM_LEN_24 {
        MEM_LEN_24::from_bits(val)
    }
}
impl From<MEM_LEN_24> for u8 {
    #[inline(always)]
    fn from(val: MEM_LEN_24) -> u8 {
        MEM_LEN_24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SMPL_EDGE {
    #[doc = "Data is sampled on the negative edge and clocked out on the positive edge."]
    NEG = 0x0,
    #[doc = "Data is sampled on the positive edge and clocked out on the negative edge."]
    POS = 0x01,
}
impl SMPL_EDGE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SMPL_EDGE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SMPL_EDGE {
    #[inline(always)]
    fn from(val: u8) -> SMPL_EDGE {
        SMPL_EDGE::from_bits(val)
    }
}
impl From<SMPL_EDGE> for u8 {
    #[inline(always)]
    fn from(val: SMPL_EDGE) -> u8 {
        SMPL_EDGE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WCLK_SRC {
    #[doc = "None ('0')."]
    NONE = 0x0,
    #[doc = "External WCLK generator, from pad."]
    EXT = 0x01,
    #[doc = "Internal WCLK generator, from module PRCM."]
    INT = 0x02,
    #[doc = "Not supported. Will give same WCLK as 'NONE' ('00')."]
    RESERVED = 0x03,
}
impl WCLK_SRC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WCLK_SRC {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WCLK_SRC {
    #[inline(always)]
    fn from(val: u8) -> WCLK_SRC {
        WCLK_SRC::from_bits(val)
    }
}
impl From<WCLK_SRC> for u8 {
    #[inline(always)]
    fn from(val: WCLK_SRC) -> u8 {
        WCLK_SRC::to_bits(val)
    }
}
