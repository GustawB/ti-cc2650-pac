#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TRACECLK_N_SWV {
    #[doc = "Internal. Only to be used through TI provided API."]
    SWV = 0x0,
    #[doc = "Internal. Only to be used through TI provided API."]
    TRACECLK = 0x01,
}
impl TRACECLK_N_SWV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TRACECLK_N_SWV {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TRACECLK_N_SWV {
    #[inline(always)]
    fn from(val: u8) -> TRACECLK_N_SWV {
        TRACECLK_N_SWV::from_bits(val)
    }
}
impl From<TRACECLK_N_SWV> for u8 {
    #[inline(always)]
    fn from(val: TRACECLK_N_SWV) -> u8 {
        TRACECLK_N_SWV::to_bits(val)
    }
}
