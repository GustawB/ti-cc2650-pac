#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTL_MODE {
    #[doc = "VIMS GPRAM mode."]
    GPRAM = 0x0,
    #[doc = "VIMS Cache mode."]
    CACHE = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "VIMS Off mode."]
    OFF = 0x03,
}
impl CTL_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTL_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTL_MODE {
    #[inline(always)]
    fn from(val: u8) -> CTL_MODE {
        CTL_MODE::from_bits(val)
    }
}
impl From<CTL_MODE> for u8 {
    #[inline(always)]
    fn from(val: CTL_MODE) -> u8 {
        CTL_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum STAT_MODE {
    #[doc = "VIMS GPRAM mode."]
    GPRAM = 0x0,
    #[doc = "VIMS Cache mode."]
    CACHE = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "VIMS Off mode."]
    OFF = 0x03,
}
impl STAT_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> STAT_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for STAT_MODE {
    #[inline(always)]
    fn from(val: u8) -> STAT_MODE {
        STAT_MODE::from_bits(val)
    }
}
impl From<STAT_MODE> for u8 {
    #[inline(always)]
    fn from(val: STAT_MODE) -> u8 {
        STAT_MODE::to_bits(val)
    }
}
