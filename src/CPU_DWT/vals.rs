#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CYCTAP {
    #[doc = "Selects bit \\[6\\] to tap."]
    BIT6 = 0x0,
    #[doc = "Selects bit \\[10\\] to tap."]
    BIT10 = 0x01,
}
impl CYCTAP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CYCTAP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CYCTAP {
    #[inline(always)]
    fn from(val: u8) -> CYCTAP {
        CYCTAP::from_bits(val)
    }
}
impl From<CYCTAP> for u8 {
    #[inline(always)]
    fn from(val: CYCTAP) -> u8 {
        CYCTAP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYNCTAP {
    #[doc = "Disabled. No synchronization packets."]
    DIS = 0x0,
    #[doc = "Tap at bit 24 of CYCCNT."]
    BIT24 = 0x01,
    #[doc = "Tap at bit 26 of CYCCNT."]
    BIT26 = 0x02,
    #[doc = "Tap at bit 28 of CYCCNT."]
    BIT28 = 0x03,
}
impl SYNCTAP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SYNCTAP {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SYNCTAP {
    #[inline(always)]
    fn from(val: u8) -> SYNCTAP {
        SYNCTAP::from_bits(val)
    }
}
impl From<SYNCTAP> for u8 {
    #[inline(always)]
    fn from(val: SYNCTAP) -> u8 {
        SYNCTAP::to_bits(val)
    }
}
