#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EN {
    #[doc = "Latches are static, meaning the current value on the IO pin is frozen by latches and kept even if GPIO module or a peripheral module is turned off."]
    STATIC = 0x0,
    #[doc = "Latches are transparent, meaning the value of the IO is directly controlled by the GPIO or peripheral value."]
    TRANSP = 0x01,
}
impl EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EN {
    #[inline(always)]
    fn from(val: u8) -> EN {
        EN::from_bits(val)
    }
}
impl From<EN> for u8 {
    #[inline(always)]
    fn from(val: EN) -> u8 {
        EN::to_bits(val)
    }
}
