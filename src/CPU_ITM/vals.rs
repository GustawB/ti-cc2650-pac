#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TSPRESCALE {
    #[doc = "No prescaling."]
    NOPRESCALING = 0x0,
    #[doc = "Divide by 4."]
    DIV4 = 0x01,
    #[doc = "Divide by 16."]
    DIV16 = 0x02,
    #[doc = "Divide by 64."]
    DIV64 = 0x03,
}
impl TSPRESCALE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TSPRESCALE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TSPRESCALE {
    #[inline(always)]
    fn from(val: u8) -> TSPRESCALE {
        TSPRESCALE::from_bits(val)
    }
}
impl From<TSPRESCALE> for u8 {
    #[inline(always)]
    fn from(val: TSPRESCALE) -> u8 {
        TSPRESCALE::to_bits(val)
    }
}
